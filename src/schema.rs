use serde::{Deserialize, Serialize};
use string_enum::StringEnum;

/// An RGB color.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IconColor {
    red: u8,
    green: u8,
    blue: u8,
}

/// A value type.
///
/// Mappings from DiamondFire types are in rustdoc on each variant.
#[derive(Clone, Eq, Hash, PartialEq, StringEnum)]
pub enum DFType {
    /// `ANY_TYPE`
    Any,
    /// `BLOCK`
    Block,
    /// `BLOCK_TAG`
    BlockTag,
    /// `DICT`
    Dictionary,
    /// `ENTITY_TYPE`
    EntityType,
    /// `ITEM`
    Item,
    /// `LIST`
    List,
    /// `LOCATION`
    Location,
    /// `NONE`
    None,
    /// `NUMBER`
    Number,
    /// `PARTICLE`
    Particle,
    /// `POTION`
    Potion,
    /// `PROJECTILE`
    Projectile,
    /// `SOUND`
    Sound,
    /// `SPAWN_EGG`
    SpawnEgg,
    /// `TEXT`
    Text,
    /// `VARIABLE`
    Variable,
    /// `VECTOR`
    Vector,
    /// `VEHICLE`
    Vehicle,
}

/// An argument to a code action.
///
/// Sometimes used for intermediate notes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Argument {
    /// The input type of the argument.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub df_type: Option<DFType>,

    /// Additional text to display with the argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether the argument can be repeated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plural: Option<bool>,
    /// Whether the argument is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    /// The description of the argument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// Any notes going along with the argument.
    ///
    /// Each element in the outer [`Vec`] is one note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<Vec<String>>>,
}

/// The visual representation of a [`CodeData`]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeIcon {
    /// The name of this code item.
    pub name: String,
    /// The item material.
    ///
    /// *Note: This is a bukkit material name.*
    pub material: String,
    /// Whether this code item is considered advanced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<bool>,

    /// A description of this code item.
    pub description: Vec<String>,
    /// An example of the usage of this code item.
    pub example: Vec<String>,
    /// Things that this code item works on/with.
    pub works_with: Vec<String>,
    /// Information about the deprecation of this code item.
    pub deprecated_note: Vec<String>,
    /// Any additional information about this code item.
    pub additional_info: Vec<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<Argument>>,

    /// The return type of this game value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_type: Option<DFType>,
    /// The description for the returned value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<Vec<String>>,

    /// The rank required to use this code item.
    pub required_rank: String,
    /// Whether this code item requires a purchase in the token shop to use.
    pub require_tokens: bool,
    /// Supposedly `true` when the following conditions are met:
    /// 1. `required_rank` is not empty
    /// 2. `require_tokens` is `true`
    ///
    /// Seems to be broken and always `false`.
    pub require_rank_and_tokens: bool,

    /// Whether this action can only be used on mobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobs_only: Option<bool>,

    /// Whether this event can be cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    /// Whether this event is automatically cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_automatically: Option<bool>,

    /// The texture data for the item if the material is `PLAYER_HEAD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    /// The color data for the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<IconColor>,

    /// A boolean encoded as 0 or 1 denoting whether this action has any tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<u8>,
    /// Unknown.
    ///
    /// Always an empty [`String`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_item: Option<String>,
}

impl CodeIcon {
    /// Returns whether this code item is considered legacy.
    ///
    /// *Note: This is determined by testing if the material is `STONE`.*
    pub fn is_legacy(&self) -> bool {
        self.material == "STONE"
    }

    /// Returns whether this code item is deprecated.
    ///
    /// *Note: This is determined by whether the deprecated note is not empty.*
    pub fn is_deprecated(&self) -> bool {
        !self.deprecated_note.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(alias = "id")]
    #[serde(alias = "particle")]
    #[serde(alias = "potion")]
    #[serde(alias = "sound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    #[serde(alias = "icon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<CodeIcon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub codeblock_name: Option<String>,
    pub sub_action_blocks: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_sub_categories: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CodeData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CodeData>>,

    #[serde(alias = "guiSlot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchasables: Option<Vec<CodeData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyType: Option<String>,
}

impl CodeData {
    pub fn is_legacy(&self) -> bool {
        match &self.item {
            Some(item) => item.is_legacy(),
            _ => false
        }
    }

    pub fn is_deprecated(&self) -> bool {
        match &self.item {
            Some(item) => item.is_deprecated(),
            _ => false
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match &self.name {
            Some(name) => name == "dynamic",
            _ => false
        }
    }
}
