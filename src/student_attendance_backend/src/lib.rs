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
        let bytes = Encode!(self).unwrap();
        if bytes.len() > Self::MAX_SIZE as usize {
            panic!("Student data exceeds maximum size");
        }
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Student {
    const MAX_SIZE: u32 = 2048; // Increased MAX_SIZE
    const IS_FIXED_SIZE: bool = false;
}

// ... (other structs remain the same)

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
        let bytes = Encode!(self).unwrap();
        if bytes.len() > Self::MAX_SIZE as usize {
            panic!("Message data exceeds maximum size");
        }
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 2048; // Increased MAX_SIZE
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

// ... (other functions remain the same)

#[ic_cdk::update]
fn send_reminder_to_student(student_id: u64, content: String, multimedia_content: Option<MultiMediaContent>, sender_id: u64) -> Result<Message, Error> {
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

#[ic_cdk::update]
fn update_message(message_id: u64, new_content: String, new_multimedia_content: Option<MultiMediaContent>) -> Result<(), Error> {
    // Validate input data
    if new_content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    // Check if the message exists
    let mut message = match _get_message(&message_id) {
        Some(msg) => msg,
        None => {
            return Err(Error::NotFound {
                msg: format!("Message with id={} not found", message_id),
            });
        }
    };

    // Verify that the caller is the sender of the message
    let caller_id = /* Get the caller's identity from the context */;
    if message.sender_id != caller_id {
        return Err(Error::InvalidInput {
            msg: "Not authorized to update this message".to_string(),
        });
    }

    // Update message content
    message.content = new_content;
    message.multimedia_content = new_multimedia_content;

    // Update message in storage
    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(message_id, message));

    Ok(())
}

fn _get_message(message_id: &u64) -> Option<&Message> {
    MESSAGE_STORAGE.with(|service| service.borrow().get(message_id))
}

// Export Candid interface
ic_cdk::export_candid!();
