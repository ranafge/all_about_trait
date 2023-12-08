// Debug // Clone // Default // PartialEq // Send & Sync

// Send is safe if your type is send between threads
// Sync is safe if your type of ref is send between threads
// these are auto trait means autometically implemented if type is contain a value Send or Sync
// Rc does not implement Send and Sync trait so our  User struct is not safe to send between threads
// Instead Rc we can use Arc smart pointer that implement Send and Sync trait.

use std::{rc::Rc, sync::Arc};
#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
struct DB {}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum Role {
    Admin,
    Standard,
    #[default]
    Guest,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct User {
    // #[default]
    id: u32,
    name: String,
    role: Role,
    #[cfg_attr(feature="serde", serde(skip))]
    db: Arc<DB>,
}

fn main() {
    let user = User {
        id: 123,
        name: "Bagdan".to_owned(),
        role: Role::Admin,
        // db: Rc::new(DB {}),
        db: Arc::new(DB {}),
    };
    println!("{:?} , {}", user, user.id);

    let user2 = User {
        id: 222,
        name: "Tagdom".to_string(),
        role: Role::Guest,
        // db: Rc::new(DB {}),
        db: Arc::new(DB {}),
    };

    println!("{}", user == user2);
    let quest = User::default();
    println!("{:?} {:?}", quest.id, quest.name);

    let quest1 = quest.clone();
    println!("{}", quest == quest1);


    let user_str = "{\"id\": 123, \"name\": \"Bagdan\", \"role\":\"Guest\"}";
    #[cfg(feature="serde")]
    let user_str_result:User = serde_json::from_str(user_str).unwrap();
    #[cfg(feature="serde")]
    println!("{:?}", user_str_result);

}

fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]

fn normal_type() {
    // Checking the User type is implemented Send , Sync trait  or not.
    is_normal::<User>()
}
