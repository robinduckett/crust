use bevy::{
    math::{Rect, Vec2},
    reflect::{std_traits::ReflectDefault, Reflect, TypeInfo, TypePath, Typed},
    utils::HashMap,
};
use nom::{
    bytes::complete::take,
    error::{Error as NomError, ErrorKind},
    multi::count,
    number::complete::{le_i32, le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult,
};

use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

/// CArchive struct as per the pattern file.
#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct CArchive {
    tag: u16,
    ob_tag: u32,
    schema: u16,
    class_name_length: u16,
    class_name: String,
}

impl CArchive {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let start_input = input.to_vec();

        let (input, tag) = loop {
            let (input, tag) = le_u16(input)?;
            println!("tag: {:?}", tag);

            if tag == 0x0000 {
                continue;
            }

            break (input, tag);
        };

        let (input, ob_tag) = if tag == 0x7fff {
            le_u32(input)?
        } else {
            let ob_tag = ((tag as u32 & 0x8000) << 16) | ((tag as u32) & 0x7fff);
            (input, ob_tag)
        };

        if (ob_tag & 0x80000000) == 0 {
            println!("ob_tag: {:?}", ob_tag.to_le_bytes());
            println!("start_input: {:?}", &start_input[0..32]);
            return Err(nom::Err::Error(NomError::new(&[0; 4], ErrorKind::Verify)));
        }

        let (input, schema) = le_u16(input)?;
        let (input, class_name_length) = le_u16(input)?;
        let (input, class_name) = take(class_name_length)(input)?;

        println!("class_name: {:?}", String::from_utf8_lossy(class_name));

        Ok((
            input,
            CArchive {
                tag,
                ob_tag,
                schema,
                class_name_length,
                class_name: String::from_utf8_lossy(class_name).to_string(),
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect)]
pub struct CImageFlags {
    gallery_class_index: u16,
    status: u8,
    width: u32,
    height: u32,
    offset: u32,
}

impl CImageFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (gallery_class_index, status, width, height, offset)) =
            tuple((le_u16, le_u8, le_u32, le_u32, le_u32))(input)?;

        Ok((
            input,
            Self {
                gallery_class_index,
                status,
                width,
                height,
                offset,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect)]
pub struct CImage {
    flags: CImageFlags,
}

impl CImage {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, flags) = CImageFlags::parse(input)?;
        Ok((input, Self { flags }))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct CGalleryFlags {
    num_images: u32,
    fsp: String,
    file_pos: u32,
    users: u32,
}

impl CGalleryFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, num_images) = le_u32(input)?;
        let (input, fsp) = take(4usize)(input)?;
        let (input, file_pos) = le_u32(input)?;
        let (input, users) = le_u32(input)?;

        Ok((
            input,
            Self {
                num_images,
                fsp: String::from_utf8_lossy(fsp).to_string(),
                file_pos,
                users,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct CGallery {
    header_or_tag: HeaderOrTag,
    flags: CGalleryFlags,
    images: Vec<CImage>,
}

impl CGallery {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        println!("CGallery::parse");
        let registration = registry.lock().unwrap().contains("CGallery");

        let (input, header_or_tag) = if !registration {
            println!("CGallery::header: {:?}", &input[0..32]);

            registry.lock().unwrap().register::<CGallery>("CGallery");
            let (input, header) = CArchive::parse(input)?;
            assert!(header.class_name == "CGallery");
            (input, HeaderOrTag::Header(header))
        } else {
            let (input, tag) = CObject::parse(input)?;
            (input, HeaderOrTag::Tag(tag))
        };

        let (input, flags) = CGalleryFlags::parse(input)?;
        let (input, images) = count(CImage::parse, flags.num_images as usize)(input)?;

        Ok((
            input,
            Self {
                header_or_tag,
                flags,
                images,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct CRect {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl From<CRect> for Rect {
    fn from(rect: CRect) -> Self {
        Rect::new(
            rect.left as f32,
            0.0 - rect.top as f32,
            rect.right as f32,
            0.0 - rect.bottom as f32,
        )
    }
}

impl CRect {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (left, top, right, bottom)) = tuple((le_u32, le_u32, le_u32, le_u32))(input)?;
        Ok((
            input,
            Self {
                left,
                top,
                right,
                bottom,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct CPoint {
    pub x: i32,
    pub y: i32,
}

impl CPoint {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (x, y)) = tuple((le_i32, le_i32))(input)?;
        Ok((input, Self { x, y }))
    }
}

impl From<&CPoint> for Vec2 {
    fn from(point: &CPoint) -> Self {
        Vec2::new(point.x as f32, point.y as f32)
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub enum RoomType {
    #[default]
    Invalid = -1,
    Indoors = 0,
    Surface = 1,
    Underwater = 2,
    Atmosphere = 3,
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub enum DropStatus {
    #[default]
    Never = 0,
    AboveFloor = 1,
    Always = 2,
}

impl From<u32> for DropStatus {
    fn from(value: u32) -> Self {
        match value {
            0 => DropStatus::Never,
            1 => DropStatus::AboveFloor,
            2 => DropStatus::Always,
            _ => DropStatus::Never,
        }
    }
}

impl Display for RoomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomType::Invalid => write!(f, "Invalid"),
            RoomType::Indoors => write!(f, "Indoors"),
            RoomType::Surface => write!(f, "Surface"),
            RoomType::Underwater => write!(f, "Underwater"),
            RoomType::Atmosphere => write!(f, "Atmosphere"),
        }
    }
}

impl RoomType {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, value) = le_i32(input)?;
        match value {
            -1 => Ok((input, RoomType::Invalid)),
            0 => Ok((input, RoomType::Indoors)),
            1 => Ok((input, RoomType::Surface)),
            2 => Ok((input, RoomType::Underwater)),
            3 => Ok((input, RoomType::Atmosphere)),
            _ => Err(nom::Err::Error(NomError::new(input, ErrorKind::Alt))),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct MapDataFlags {
    map_is_wrappable: u32,
    time_of_day: u32,
    day_in_year: u32,
    year: u32,
}

impl MapDataFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (map_is_wrappable, time_of_day, day_in_year, year)) =
            tuple((le_u32, le_u32, le_u32, le_u32))(input)?;
        Ok((
            input,
            Self {
                map_is_wrappable,
                time_of_day,
                day_in_year,
                year,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct CObject {
    tag: u16,
}

impl CObject {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, tag) = le_u16(input)?;
        Ok((input, Self { tag }))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct DoorPointerArrayItem {
    header_or_tag: HeaderOrTag,
    pub amount_open: u8,
    pub room_id: u32,
}

impl DoorPointerArrayItem {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let registration = registry.lock().unwrap().contains("CDoor");

        println!("CDoor::registration: {:?}", registration);

        let (input, header_or_tag) = if !registration {
            registry
                .lock()
                .unwrap()
                .register::<DoorPointerArrayItem>("CDoor");
            let (input, header) = CArchive::parse(input)?;
            println!("CDoor::header: {:?}", header);
            assert!(header.class_name == "CDoor");
            (input, HeaderOrTag::Header(header))
        } else {
            let (input, tag) = CObject::parse(input)?;
            (input, HeaderOrTag::Tag(tag))
        };

        let (input, amount_open) = le_u8(input)?;
        let (input, room_id) = le_u32(input)?;

        println!("DoorPointerArrayItem::room_id: {}", room_id);
        Ok((
            input,
            Self {
                header_or_tag,
                amount_open,
                room_id,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct DoorPointerArray {
    size: u16,
    pub doors: Vec<DoorPointerArrayItem>,
}

impl DoorPointerArray {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let (input, size) = le_u16(input)?;
        println!("Door::size: {}", size);

        if size > 2000 {
            println!("Door::size: {}", size);
            return Err(nom::Err::Error(NomError::new(input, ErrorKind::Count)));
        }
        let mut inputs = input;
        let mut doors = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let (next_input, door) = DoorPointerArrayItem::parse(inputs, registry)?;
            doors.push(door);
            inputs = next_input;
        }
        Ok((inputs, Self { size, doors }))
    }
}

// need to make the door array an array of an array of door pointer array items

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct Doors {
    pub doors: Vec<DoorPointerArray>,
}

impl Doors {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let mut inputs = input;
        let mut doors = Vec::with_capacity(4);
        for _ in 0..4 {
            let (next_input, door) = DoorPointerArray::parse(inputs, registry)?;
            doors.push(door);
            inputs = next_input;
        }
        Ok((inputs, Self { doors }))
    }
}

#[repr(u8)]
#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub enum BacteriaState {
    #[default]
    NotPresent = 0,
    Dormant = 1,
    Active = 2,
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct BacteriaFlags {
    state: BacteriaState,
}

impl BacteriaFlags {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, byte) = le_u8(input)?;
        let state_value = byte & 0b11; // Assuming 2 bits for state
        let state = match state_value {
            0 => BacteriaState::NotPresent,
            1 => BacteriaState::Dormant,
            2 => BacteriaState::Active,
            _ => {
                return Err(nom::Err::Error(NomError::new(input, ErrorKind::Alt)));
            }
        };
        Ok((input, Self { state }))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct Bacteria {
    flags: BacteriaFlags,
    antigen: u8,
    fatal_level: u8,
    infect_level: u8,
    toxins: [u8; 4],
}

impl Bacteria {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, flags) = BacteriaFlags::parse(input)?;
        let (input, antigen) = le_u8(input)?;
        let (input, fatal_level) = le_u8(input)?;
        let (input, infect_level) = le_u8(input)?;
        let (input, toxins) = take(4usize)(input)?;
        Ok((
            input,
            Self {
                flags,
                antigen,
                fatal_level,
                infect_level,
                toxins: toxins.try_into().unwrap(),
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct CPointArray {
    size: u16,
    pub points: Vec<CPoint>,
}

impl From<&CPointArray> for Vec<Vec2> {
    fn from(point_array: &CPointArray) -> Self {
        point_array.points.iter().map(Vec2::from).collect()
    }
}

impl CPointArray {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, size) = le_u16(input)?;
        if size > 2000 {
            println!("CPointArray::size: {}", size);
            return Err(nom::Err::Error(NomError::new(input, ErrorKind::Count)));
        }
        let (input, points) = count(CPoint::parse, size as usize)(input)?;
        Ok((input, Self { size, points }))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct CString {
    size: u8,
    string: String,
}

impl CString {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, size) = le_u8(input)?;
        let (input, string) = take(size as usize)(input)?;
        Ok((
            input,
            Self {
                size,
                string: String::from_utf8_lossy(string).to_string(),
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub enum HeaderOrTag {
    Header(CArchive),
    Tag(CObject),
    #[default]
    None,
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct RoomPointer {
    pub header_or_tag: HeaderOrTag,
    pub room_id: u32,
    pub map_class_index: u16,
    pub rect: CRect,
    pub doors: Doors,
    pub room_type: RoomType,
    pub floor_value: u8,
    pub inorganic_nutrient: u8,
    pub organic_nutrient: u8,
    pub temperature: u8,
    pub heat_source: i32,
    pub pressure: u8,
    pub pressure_source: i32,
    pub wind: CPoint,
    pub light: u8,
    pub light_source: i32,
    pub radiation: u8,
    pub radiation_source: i32,
    pub bacterium: Vec<Bacteria>,
    pub surface_points: CPointArray,
    pub visited: u32,
    pub music_track: CString,
    pub drop_status: DropStatus,
}

impl RoomPointer {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let registration = registry.lock().unwrap().contains("CRoom");

        let (input, header_or_tag) = if !registration {
            registry.lock().unwrap().register::<RoomPointer>("CRoom");

            let (input, header) = CArchive::parse(input)?;
            assert!(header.class_name == "CRoom");
            (input, HeaderOrTag::Header(header))
        } else {
            let (input, tag) = CObject::parse(input)?;
            (input, HeaderOrTag::Tag(tag))
        };

        let (input, room_id) = le_u32(input)?;
        let (input, map_class_index) = le_u16(input)?;
        let (input, rect) = CRect::parse(input)?;
        println!("RoomPointer::rect: {:?}", rect);
        let (input, doors) = Doors::parse(input, registry)?;
        let (input, room_type) = RoomType::parse(input)?;
        let (input, floor_value) = le_u8(input)?;
        let (input, inorganic_nutrient) = le_u8(input)?;
        let (input, organic_nutrient) = le_u8(input)?;
        let (input, temperature) = le_u8(input)?;
        let (input, heat_source) = le_i32(input)?;
        let (input, pressure) = le_u8(input)?;
        let (input, pressure_source) = le_i32(input)?;
        let (input, wind) = CPoint::parse(input)?;
        let (input, light) = le_u8(input)?;
        let (input, light_source) = le_i32(input)?;
        let (input, radiation) = le_u8(input)?;
        let (input, radiation_source) = le_i32(input)?;
        let (input, bacterium) = count(Bacteria::parse, 100)(input)?;
        let (input, surface_points) = CPointArray::parse(input)?;
        let (input, visited) = le_u32(input)?;
        let (input, music_track) = CString::parse(input)?;
        let (input, drop_status) = le_u32(input)?;

        Ok((
            input,
            Self {
                header_or_tag,
                room_id,
                map_class_index,
                rect,
                doors,
                room_type,
                floor_value,
                inorganic_nutrient,
                organic_nutrient,
                temperature,
                heat_source,
                pressure,
                pressure_source,
                wind,
                light,
                light_source,
                radiation,
                radiation_source,
                bacterium,
                surface_points,
                visited,
                music_track,
                drop_status: drop_status.into(),
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Rooms {
    pub count: u32,
    pub rooms: Vec<RoomPointer>,
}

impl Rooms {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let (input, len) = le_u32(input)?;
        println!("Rooms::len: {:?}", len);
        if len > 2000 {
            println!("Rooms::len: {}", len);
            return Err(nom::Err::Error(NomError::new(input, ErrorKind::Count)));
        }

        let (input, rooms) = {
            let mut rooms = Vec::with_capacity(len as usize);

            let mut inputs = input;

            for _ in 0..len {
                let (next_input, room) = RoomPointer::parse(inputs, registry)?;
                rooms.push(room);

                inputs = next_input
            }

            (inputs, rooms)
        };

        Ok((input, Self { count: len, rooms }))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct MapData {
    header: CArchive,
    flags: MapDataFlags,
    tile_gallery: CGallery,
    pub rooms: Rooms,
}

impl MapData {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        println!("MapData::header: {:?}", &input[0..32]);
        let (input, header) = CArchive::parse(input)?;
        assert!(header.class_name == "MapData");
        let (input, flags) = MapDataFlags::parse(input)?;
        println!("MapData::flags: {:?}", flags);
        let (input, tile_gallery) = CGallery::parse(input, registry)?;
        let (input, rooms) = Rooms::parse(input, registry)?;

        Ok((
            input,
            Self {
                header,
                flags,
                tile_gallery,
                rooms,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
pub struct Classifier {
    family_genus: u16,
    species_event: u32,
}

impl Classifier {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, family_genus) = le_u16(input)?;
        let (input, species_event) = le_u32(input)?;
        Ok((
            input,
            Self {
                family_genus,
                species_event,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub enum MovementStatus {
    #[default]
    Autonomous = 0,
    MouseDriven = 1,
    Floating = 2,
    InVehicle = 3,
    Carried = 4,
}

impl MovementStatus {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, value) = le_u8(input)?;
        match value {
            0 => Ok((input, MovementStatus::Autonomous)),
            1 => Ok((input, MovementStatus::MouseDriven)),
            2 => Ok((input, MovementStatus::Floating)),
            3 => Ok((input, MovementStatus::InVehicle)),
            4 => Ok((input, MovementStatus::Carried)),
            _ => Err(nom::Err::Error(NomError::new(input, ErrorKind::Alt))),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Attributes {
    carryable: bool,
    mouseable: bool,
    activatable: bool,
    container: bool,
    invisible: bool,
    floatable: bool,
    has_boundaries: bool,
    suffers_gravity: bool,
}

impl Attributes {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, byte) = le_u8(input)?;
        Ok((
            input,
            Self {
                carryable: (byte & 0b00000001) != 0,
                mouseable: (byte & 0b00000010) != 0,
                activatable: (byte & 0b00000100) != 0,
                container: (byte & 0b00001000) != 0,
                invisible: (byte & 0b00010000) != 0,
                floatable: (byte & 0b00100000) != 0,
                has_boundaries: (byte & 0b01000000) != 0,
                suffers_gravity: (byte & 0b10000000) != 0,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Objvars {
    var: u32,
}

impl Objvars {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, var) = le_u32(input)?;
        Ok((input, Self { var }))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Script {
    classifier: Classifier,
    script_body: CString,
}

impl Script {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, classifier) = Classifier::parse(input)?;
        let (input, script_body) = CString::parse(input)?;
        Ok((
            input,
            Self {
                classifier,
                script_body,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Object {
    header_or_tag: HeaderOrTag,
    classifier: Classifier,
    id: i32,
    movement_status: MovementStatus,
    attributes: Attributes,
    limit: CRect,
    vehicle_ptr: u16,
    active: u8,
    obj_gallery: CGallery,
    timer_rate: u32,
    timer: u32,
    obj_pointer: u16,
    active_sound: u32,
    vars: Vec<Objvars>,
    min_door_size: u8,
    range: i32,
    falling_object_index: i32,
    acceleration_due_to_gravity: i32,
    velocity: CPoint,
    restitution: i32,
    aerodynamic: i32,
    current_room: u16,
    wall_last_collided: u32,
    threat: u8,
    running: u8,
    num_caos_scripts: u32,
    scripts: Vec<Script>,
}

impl Object {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let registration = registry.lock().unwrap().contains("Object");

        let (input, header_or_tag) = if !registration {
            registry.lock().unwrap().register::<Object>("Object");

            println!("Object::header: {:?}", &input[0..32]);

            let (input, header) = CArchive::parse(input)?;
            assert!(header.class_name == "Object");
            (input, HeaderOrTag::Header(header))
        } else {
            let (input, tag) = CObject::parse(input)?;
            (input, HeaderOrTag::Tag(tag))
        };

        let (input, classifier) = Classifier::parse(input)?;
        let (input, id) = le_i32(input)?;
        let (input, movement_status) = MovementStatus::parse(input)?;
        let (input, attributes) = Attributes::parse(input)?;
        let (input, limit) = CRect::parse(input)?;
        let (input, vehicle_ptr) = le_u16(input)?;
        let (input, active) = le_u8(input)?;
        let (input, obj_gallery) = CGallery::parse(input, registry)?;
        let (input, timer_rate) = le_u32(input)?;
        let (input, timer) = le_u32(input)?;
        let (input, obj_pointer) = le_u16(input)?;
        let (input, active_sound) = le_u32(input)?;
        let (input, vars) = count(Objvars::parse, 100)(input)?;
        let (input, min_door_size) = le_u8(input)?;
        let (input, range) = le_i32(input)?;
        let (input, falling_object_index) = le_i32(input)?;
        let (input, acceleration_due_to_gravity) = le_i32(input)?;
        let (input, velocity) = CPoint::parse(input)?;
        let (input, restitution) = le_i32(input)?;
        let (input, aerodynamic) = le_i32(input)?;
        let (input, current_room) = le_u16(input)?;
        let (input, wall_last_collided) = le_u32(input)?;
        let (input, threat) = le_u8(input)?;
        let (input, running) = le_u8(input)?;
        let (input, num_caos_scripts) = le_u32(input)?;
        if num_caos_scripts > 2000 {
            println!("Object::num_caos_scripts: {}", num_caos_scripts);
            return Err(nom::Err::Error(NomError::new(input, ErrorKind::Count)));
        }
        let (input, scripts) = count(Script::parse, num_caos_scripts as usize)(input)?;

        Ok((
            input,
            Self {
                header_or_tag,
                classifier,
                id,
                movement_status,
                attributes,
                limit,
                vehicle_ptr,
                active,
                obj_gallery,
                timer_rate,
                timer,
                obj_pointer,
                active_sound,
                vars,
                min_door_size,
                range,
                falling_object_index,
                acceleration_due_to_gravity,
                velocity,
                restitution,
                aerodynamic,
                current_room,
                wall_last_collided,
                threat,
                running,
                num_caos_scripts,
                scripts,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Entity {
    // Define fields as needed
    header_or_tag: HeaderOrTag,
    gallery_tag: u16,
    image_index: u8,
    base_index: u8,
    plane: i32,
    world_x: i32,
    world_y: i32,
    anim: String,
}

impl Entity {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let registration = registry.lock().unwrap().contains("Entity");

        let (input, header_or_tag) = if !registration {
            registry.lock().unwrap().register::<Entity>("Entity");

            println!("Entity::header: {:?}", &input[0..32]);

            let (input, header) = CArchive::parse(input)?;
            assert!(header.class_name == "Entity");
            (input, HeaderOrTag::Header(header))
        } else {
            let (input, tag) = CObject::parse(input)?;
            (input, HeaderOrTag::Tag(tag))
        };

        let (input, gallery_tag) = le_u16(input)?;
        let (input, image_index) = le_u8(input)?;
        let (input, base_index) = le_u8(input)?;
        let (input, plane) = le_i32(input)?;
        let (input, world_x) = le_i32(input)?;
        let (input, world_y) = le_i32(input)?;

        let (input, flag) = le_u8(input)?;

        let (input, anim) = if flag == 1 {
            let (input, anim) = take(99usize)(input)?;
            let anim = String::from_utf8(anim.to_vec()).ok().unwrap();
            (input, anim)
        } else {
            (input, "".to_string())
        };

        Ok((
            input,
            Self {
                header_or_tag,
                gallery_tag,
                image_index,
                base_index,
                plane,
                world_x,
                world_y,
                anim,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct SimpleObject {
    pub header_or_tag: HeaderOrTag,
    pub classifier: Classifier,
    pub id: i32,
    pub movement_status: MovementStatus,
    pub attributes: Attributes,
    pub limit: CRect,
    pub vehicle_ptr: u16,
    pub active: u8,
    pub obj_gallery: CGallery,
    pub timer_rate: u32,
    pub timer: u32,
    pub obj_pointer: u16,
    pub active_sound: u32,
    pub vars: Vec<Objvars>,
    pub min_door_size: u8,
    pub range: i32,
    pub falling_object_index: i32,
    pub acceleration_due_to_gravity: i32,
    pub velocity: CPoint,
    pub restitution: i32,
    pub aerodynamic: i32,
    pub current_room: u16,
    pub wall_last_collided: u32,
    pub threat: u8,
    pub running: u8,
    pub num_caos_scripts: u32,
    pub scripts: Vec<Script>,
    pub entity: Entity,
    pub gallery_tag: u16,
    pub image_index: u8,
    pub base_index: u8,
    pub plane: i32,
    pub world_x: i32,
    pub world_y: i32,
    pub flag: u8,
    pub anim: String,
    pub normal_plane: i32,
    pub click: [u8; 3],
    pub touch: u8,
    pub pickup_handle: CPointArray,
    pub pickup_point: CPointArray,
}

impl SimpleObject {
    fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        let registration = registry.lock().unwrap().contains("SimpleObject");

        let (input, header_or_tag) = if !registration {
            registry
                .lock()
                .unwrap()
                .register::<SimpleObject>("SimpleObject");

            println!("SimpleObject::header: {:?}", &input[0..32]);

            let (input, header) = CArchive::parse(input)?;

            assert!(header.class_name == "SimpleObject");

            (input, HeaderOrTag::Header(header))
        } else {
            let (input, tag) = CObject::parse(input)?;
            (input, HeaderOrTag::Tag(tag))
        };

        let (input, classifier) = Classifier::parse(input)?;
        let (input, id) = le_i32(input)?;
        let (input, movement_status) = MovementStatus::parse(input)?;
        let (input, attributes) = Attributes::parse(input)?;
        let (input, limit) = CRect::parse(input)?;
        let (input, vehicle_ptr) = le_u16(input)?;
        let (input, active) = le_u8(input)?;
        let (input, obj_gallery) = CGallery::parse(input, registry)?;
        let (input, timer_rate) = le_u32(input)?;
        let (input, timer) = le_u32(input)?;
        let (input, obj_pointer) = le_u16(input)?;
        let (input, active_sound) = le_u32(input)?;
        let (input, vars) = count(Objvars::parse, 100)(input)?;
        let (input, min_door_size) = le_u8(input)?;
        let (input, range) = le_i32(input)?;
        let (input, falling_object_index) = le_i32(input)?;
        let (input, acceleration_due_to_gravity) = le_i32(input)?;
        let (input, velocity) = CPoint::parse(input)?;
        let (input, restitution) = le_i32(input)?;
        let (input, aerodynamic) = le_i32(input)?;
        let (input, current_room) = le_u16(input)?;
        let (input, wall_last_collided) = le_u32(input)?;
        let (input, threat) = le_u8(input)?;
        let (input, running) = le_u8(input)?;
        let (input, num_caos_scripts) = le_u32(input)?;
        if num_caos_scripts > 2000 {
            println!("SimpleObject::num_caos_scripts: {}", num_caos_scripts);
            return Err(nom::Err::Error(NomError::new(input, ErrorKind::Count)));
        }
        let (input, scripts) = count(Script::parse, num_caos_scripts as usize)(input)?;

        let (input, entity) = Entity::parse(input, registry)?;

        let (input, gallery_tag) = le_u16(input)?;
        let (input, image_index) = le_u8(input)?;
        let (input, base_index) = le_u8(input)?;
        let (input, plane) = le_i32(input)?;
        let (input, world_x) = le_i32(input)?;
        let (input, world_y) = le_i32(input)?;
        let (input, flag) = le_u8(input)?;
        let (input, anim) = take(99usize)(input)?;
        let anim = String::from_utf8(anim.to_vec()).ok().unwrap();
        let (input, normal_plane) = le_i32(input)?;
        let (input, click) = take(3usize)(input)?;
        let click = [click[0], click[1], click[2]];
        let (input, touch) = le_u8(input)?;
        let (input, pickup_handle) = CPointArray::parse(input)?;
        let (input, pickup_point) = CPointArray::parse(input)?;

        Ok((
            input,
            Self {
                header_or_tag,
                classifier,
                id,
                movement_status,
                attributes,
                limit,
                vehicle_ptr,
                active,
                obj_gallery,
                timer_rate,
                timer,
                obj_pointer,
                active_sound,
                vars,
                min_door_size,
                range,
                falling_object_index,
                acceleration_due_to_gravity,
                velocity,
                restitution,
                aerodynamic,
                current_room,
                wall_last_collided,
                threat,
                running,
                num_caos_scripts,
                scripts,
                entity,
                gallery_tag,
                image_index,
                base_index,
                plane,
                world_x,
                world_y,
                flag,
                anim,
                normal_plane,
                click,
                touch,
                pickup_handle,
                pickup_point,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug, Reflect, Default)]
#[reflect(Default)]
pub struct Doc {
    pub map: MapData,
    pub num_objects: u32,
    pub objects: Vec<Object>,
    pub num_scenery: u32,
    pub simple_object_pointer: Vec<SimpleObject>,
}

impl Doc {
    pub fn parse<'a>(
        input: &'a [u8],
        registry: &mut Arc<Mutex<ClassRegistry>>,
    ) -> IResult<&'a [u8], Self> {
        println!("Doc::parse: {:?}", &input[0..32]);
        let (input, map) = MapData::parse(input, registry)?;
        let (input, num_objects) = le_u32(input)?;
        if num_objects > 2000 {
            println!("num_objects: {}", num_objects);
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Count,
            )));
        }
        let mut objects = Vec::new();
        let mut inputs = input;

        for _ in 0..num_objects {
            let (next_input, object) = Object::parse(inputs, registry)?;
            objects.push(object);
            inputs = next_input;
        }

        let input = inputs;

        let (input, num_scenery) = le_u32(input)?;
        if num_scenery > 2000 {
            println!("num_scenery: {}", num_scenery);
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Count,
            )));
        }
        let mut simple_object_pointer = Vec::new();
        let mut inputs = input;

        for _ in 0..num_scenery {
            let (next_input, simple_object) = SimpleObject::parse(inputs, registry)?;
            simple_object_pointer.push(simple_object);
            inputs = next_input;
        }

        Ok((
            inputs,
            Self {
                map,
                num_objects,
                objects,
                num_scenery,
                simple_object_pointer,
            },
        ))
    }
}

#[derive(Debug)]
pub struct ClassRegistry {
    classes: HashMap<String, &'static TypeInfo>,
}

impl ClassRegistry {
    pub fn empty() -> Self {
        Self {
            classes: HashMap::new(),
        }
    }

    pub fn register<T: Reflect + Typed + TypePath>(&mut self, class_name: &str) {
        self.classes.insert(class_name.to_string(), T::type_info());
    }

    pub fn contains(&self, class_name: &str) -> bool {
        self.classes.contains_key(class_name)
    }
}

#[test]
fn test_parse_sfc() {
    let buf = include_bytes!("../../assets/test.sfc");
    let mut registry: Arc<Mutex<ClassRegistry>> = Arc::new(Mutex::new(ClassRegistry::empty()));

    let doc = Doc::parse(buf, &mut registry);

    match doc {
        Ok((_, doc)) => println!("{:?}", doc.map.rooms.rooms[0]),
        Err(e) => println!(
            "Error parsing SFC file: {:?} - error",
            e.map(|err| err.code)
        ),
    }
}
