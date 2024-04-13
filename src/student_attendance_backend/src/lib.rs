#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define MultimediaContent struct for multimedia communication
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct MultiMediaContent {
    image_url: Option<String>,
    video_url: Option<String>,
    audio_url: Option<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Student {
    id: u64,
    name: String,
    contact_details: String,
    attendance_history: String,
}

impl Storable for Student {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Student {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Lecture {
    id: u64,
    student_id: u64,
    lecturer_id: u64,
    date_time: u64,
    topic: String,
    multimedia_content: Option<MultiMediaContent>,
}

impl Storable for Lecture {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Lecture {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct AttendanceRecord {
    id: u64,
    student_id: u64,
    attendance_status: String,
}

impl Storable for AttendanceRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for AttendanceRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STUDENT_STORAGE: RefCell<StableBTreeMap<u64, Student, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static LECTURE_STORAGE: RefCell<StableBTreeMap<u64, Lecture, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ATTENDANCE_RECORD_STORAGE: RefCell<StableBTreeMap<u64, AttendanceRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static MESSAGE_STORAGE: RefCell<StableBTreeMap<u64, Message, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

#[ic_cdk::query]
fn get_student(student_id: u64) -> Result<Student, Error> {
    match _get_student(&student_id) {
        Some(student) => Ok(student),
        None => Err(Error::NotFound {
            msg: format!("Student with id={} not found", student_id),
        }),
    }
}

#[ic_cdk::update]
fn register_student(name: String, contact_details: String, attendance_history: String) -> Result<Student, Error> {
    // Validate input data
    if name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let student = Student { id, name, contact_details, attendance_history };

    STUDENT_STORAGE.with(|service| service.borrow_mut().insert(id, student.clone()));
    Ok(student)
}

#[ic_cdk::query]
fn get_lecture(lecture_id: u64) -> Result<Lecture, Error> {
    match _get_lecture(&lecture_id) {
        Some(lecture) => Ok(lecture),
        None => Err(Error::NotFound {
            msg: format!("Lecture with id={} not found", lecture_id),
        }),
    }
}

#[ic_cdk::update]
fn schedule_lecture(student_id: u64, lecturer_id: u64, date_time: u64, topic: String, multimedia_content: Option<MultiMediaContent>) -> Result<Lecture, Error> {
    // Validate input data
    if topic.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Topic cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let lecture = Lecture {
        id,
        student_id,
        lecturer_id,
        date_time,
        topic,
        multimedia_content,
    };

    LECTURE_STORAGE.with(|service| service.borrow_mut().insert(id, lecture.clone()));
    Ok(lecture)
}

#[ic_cdk::query]
fn get_attendance_record(record_id: u64) -> Result<AttendanceRecord, Error> {
    match _get_attendance_record(&record_id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("Attendance record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
fn update_student(student_id: u64, name: String, contact_details: String, attendance_history: String) -> Result<Student, Error> {
    // Validate input data
    if name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    let updated_student = Student { id: student_id, name, contact_details, attendance_history };

    // Update student in storage
    match STUDENT_STORAGE.with(|service| service.borrow_mut().insert(student_id, updated_student.clone())) {
        Some(_) => Ok(updated_student),
        None => Err(Error::NotFound {
            msg: format!("Student with id={} not found", student_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_student(student_id: u64) -> Result<(), Error> {
    // Remove student from storage
    match STUDENT_STORAGE.with(|service| service.borrow_mut().remove(&student_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Student with id={} not found", student_id),
        }),
    }
}

#[ic_cdk::query]
fn list_students() -> Vec<Student> {
    STUDENT_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, student)| student.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_lecture(lecture_id: u64, student_id: u64, lecturer_id: u64, date_time: u64, topic: String, multimedia_content: Option<MultiMediaContent>) -> Result<Lecture, Error> {
    // Validate input data
    if topic.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Topic cannot be empty".to_string(),
        });
    }

    let updated_lecture = Lecture {
        id: lecture_id,
        student_id,
        lecturer_id,
        date_time,
        topic,
        multimedia_content,
    };

    // Update lecture in storage
    match LECTURE_STORAGE.with(|service| service.borrow_mut().insert(lecture_id, updated_lecture.clone())) {
        Some(_) => Ok(updated_lecture),
        None => Err(Error::NotFound {
            msg: format!("Lecture with id={} not found", lecture_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_lecture(lecture_id: u64) -> Result<(), Error> {
    // Remove lecture from storage
    match LECTURE_STORAGE.with(|service| service.borrow_mut().remove(&lecture_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Lecture with id={} not found", lecture_id),
        }),
    }
}

#[ic_cdk::query]
fn list_lectures() -> Vec<Lecture> {
    LECTURE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, lecture)| lecture.clone())
            .collect()
    })
}

#[ic_cdk::update]
fn update_attendance_record(record_id: u64, student_id: u64, attendance_status: String) -> Result<AttendanceRecord, Error> {
    let updated_record = AttendanceRecord {
        id: record_id,
        student_id,
        attendance_status,
    };

    // Update attendance record in storage
    match ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow_mut().insert(record_id, updated_record.clone())) {
        Some(_) => Ok(updated_record),
        None => Err(Error::NotFound {
            msg: format!("Attendance record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_attendance_record(record_id: u64) -> Result<(), Error> {
    // Remove attendance record from storage
    match ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow_mut().remove(&record_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Attendance record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::query]
fn list_attendance_records() -> Vec<AttendanceRecord> {
    ATTENDANCE_RECORD_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect()
    })
}

fn _get_student(student_id: &u64) -> Option<Student> {
    STUDENT_STORAGE.with(|service| service.borrow().get(student_id))
}

fn _get_lecture(lecture_id: &u64) -> Option<Lecture> {
    LECTURE_STORAGE.with(|service| service.borrow().get(lecture_id))
}

fn _get_attendance_record(record_id: &u64) -> Option<AttendanceRecord> {
    ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow().get(record_id))
}

#[ic_cdk::update]
fn send_reminder_to_student(student_id: u64, content: String, multimedia_content: Option<MultiMediaContent>) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Reminder content cannot be empty".to_string(),
        });
    }

    // Check if the student exists
    if _get_student(&student_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Student with id={} not found", student_id),
        });
    }

    // Get the sender ID (could be a system ID or a lecturer ID)
    let sender_id = 0; // You can change this based on your system design

    // Construct the message
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let message = Message {
        id,
        sender_id,
        receiver_id: student_id,
        content,
        multimedia_content,
    };

    // Store the message
    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(id, message.clone()));

    Ok(message)
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Message {
    id: u64,
    sender_id: u64,
    receiver_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
}

impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[ic_cdk::update]
fn update_message(message_id: u64, sender_id: u64, receiver_id: u64, content: String, multimedia_content: Option<MultiMediaContent>) -> Result<Message, Error> {
    // Validate input data
    if content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    let updated_message = Message {
        id: message_id,
        sender_id,
        receiver_id,
        content,
        multimedia_content,
    };

    // Update message in storage
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(message_id, updated_message.clone())) {
        Some(_) => Ok(updated_message),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_message(message_id: u64) -> Result<(), Error> {
    // Remove message from storage
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().remove(&message_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

#[ic_cdk::query]
fn list_messages() -> Vec<Message> {
    MESSAGE_STORAGE.with(|service| {
        service
            .borrow()
            .iter()
            .map(|(_, message)| message.clone())
            .collect()
    })
}

fn _get_message(message_id: &u64) -> Option<Message> {
    MESSAGE_STORAGE.with(|service| service.borrow().get(message_id))
}

// Export Candid interface
ic_cdk::export_candid!();
