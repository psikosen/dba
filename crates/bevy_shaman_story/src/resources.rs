use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// INSTRUMENT CHOICE
// ============================================================================

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentChoice {
    #[default]
    NotChosen,
    Kora,
    Ngoni,
}

impl InstrumentChoice {
    /// Kora: stronger stability/purification, easier control
    pub fn stability_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 1.3,
            InstrumentChoice::Ngoni => 0.9,
            InstrumentChoice::NotChosen => 1.0,
        }
    }

    /// Ngoni: higher damage, stronger state pushing
    pub fn damage_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 0.9,
            InstrumentChoice::Ngoni => 1.3,
            InstrumentChoice::NotChosen => 1.0,
        }
    }

    /// Kora: better control, Ngoni: higher stamina costs
    pub fn stamina_cost_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 0.8,
            InstrumentChoice::Ngoni => 1.2,
            InstrumentChoice::NotChosen => 1.0,
        }
    }

    /// Kora: easier to control minions
    pub fn obedience_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 1.2,
            InstrumentChoice::Ngoni => 0.8,
            InstrumentChoice::NotChosen => 1.0,
        }
    }
}

// ============================================================================
// BROTHER CLEANSING PROGRESS
// ============================================================================

#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BrotherCleansingProgress {
    pub fights_completed: u8,
    pub total_fights: u8,
}

impl BrotherCleansingProgress {
    pub fn new() -> Self {
        Self {
            fights_completed: 0,
            total_fights: 4,
        }
    }

    pub fn complete_fight(&mut self) {
        if self.fights_completed < self.total_fights {
            self.fights_completed += 1;
        }
    }

    pub fn is_fully_cleansed(&self) -> bool {
        self.fights_completed >= self.total_fights
    }

    pub fn corruption_percentage(&self) -> f32 {
        1.0 - (self.fights_completed as f32 / self.total_fights as f32)
    }
}

// ============================================================================
// AFRICAN NAMES DATABASE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterType {
    MainCharacter,
    Brother,
    Companion,
    Boss,
    Elder,
    Merchant,
    Farmer,
    Hunter,
    Child,
    SpiritualLeader,
    Villager,
}

#[derive(Resource)]
pub struct AfricanNamesDB {
    pub names: HashMap<String, (CharacterType, String, String)>, // name -> (type, origin, meaning)
}

impl Default for AfricanNamesDB {
    fn default() -> Self {
        let mut names = HashMap::new();

        // MAIN CHARACTERS
        names.insert("Kwame".to_string(), (CharacterType::MainCharacter, "Akan, Ghana".to_string(), "Born on Saturday".to_string()));

        // BROTHERS
        names.insert("Kofi".to_string(), (CharacterType::Brother, "Akan, Ghana".to_string(), "Born on Friday".to_string()));
        names.insert("Kwesi".to_string(), (CharacterType::Brother, "Akan, Ghana".to_string(), "Born on Sunday".to_string()));
        names.insert("Kwadwo".to_string(), (CharacterType::Brother, "Akan, Ghana".to_string(), "Born on Monday".to_string()));
        names.insert("Yaw".to_string(), (CharacterType::Brother, "Akan, Ghana".to_string(), "Born on Thursday".to_string()));

        // COMPANIONS
        names.insert("Amara".to_string(), (CharacterType::Companion, "Igbo, Nigeria".to_string(), "Grace".to_string()));
        names.insert("Zuri".to_string(), (CharacterType::Companion, "Swahili, East Africa".to_string(), "Beautiful".to_string()));
        names.insert("Jabari".to_string(), (CharacterType::Companion, "Swahili, East Africa".to_string(), "Brave one".to_string()));

        // BOSSES
        names.insert("Anansi".to_string(), (CharacterType::Boss, "Akan, Ghana".to_string(), "Spider - Trickster Spirit".to_string()));
        names.insert("Mami Wata".to_string(), (CharacterType::Boss, "Pan-African".to_string(), "Mother Water".to_string()));
        names.insert("Jengu".to_string(), (CharacterType::Boss, "Cameroon".to_string(), "Water spirit".to_string()));
        names.insert("Oya".to_string(), (CharacterType::Boss, "Yoruba, Nigeria".to_string(), "She Tore - Storm Spirit".to_string()));
        names.insert("Eshu".to_string(), (CharacterType::Boss, "Yoruba, Nigeria".to_string(), "Trickster - Crossroads Spirit".to_string()));
        names.insert("Chuma".to_string(), (CharacterType::Boss, "Swahili, East Africa".to_string(), "Wealth/Iron - Iron Sentinel".to_string()));
        names.insert("Nkrumah".to_string(), (CharacterType::Boss, "Akan, Ghana".to_string(), "Ninth born - Ancient Protector".to_string()));
        names.insert("Sundiata".to_string(), (CharacterType::Boss, "Mandinka, Mali".to_string(), "Lion Prince - Warrior King Spirit".to_string()));
        names.insert("Shango".to_string(), (CharacterType::Boss, "Yoruba, Nigeria".to_string(), "Thunder - God of Thunder and Lightning".to_string()));

        // ELDERS
        names.insert("Chike".to_string(), (CharacterType::Elder, "Igbo, Nigeria".to_string(), "Power of God".to_string()));
        names.insert("Nala".to_string(), (CharacterType::Elder, "Swahili, East Africa".to_string(), "Gift - Wise Woman".to_string()));
        names.insert("Tendaji".to_string(), (CharacterType::Elder, "Swahili, East Africa".to_string(), "Makes things happen".to_string()));

        // MERCHANTS
        names.insert("Kamari".to_string(), (CharacterType::Merchant, "Swahili, East Africa".to_string(), "Like the moon".to_string()));
        names.insert("Zola".to_string(), (CharacterType::Merchant, "Zulu, South Africa".to_string(), "Quiet, tranquil - Herbalist".to_string()));
        names.insert("Thabo".to_string(), (CharacterType::Merchant, "Sotho, South Africa".to_string(), "Joy - Blacksmith".to_string()));
        names.insert("Ife".to_string(), (CharacterType::Merchant, "Yoruba, Nigeria".to_string(), "Love - Weaver".to_string()));
        names.insert("Sefu".to_string(), (CharacterType::Merchant, "Swahili, East Africa".to_string(), "Sword - Weapon trader".to_string()));
        names.insert("Azizi".to_string(), (CharacterType::Merchant, "Swahili, East Africa".to_string(), "Precious - Jeweler".to_string()));

        // FARMERS
        names.insert("Adanna".to_string(), (CharacterType::Farmer, "Igbo, Nigeria".to_string(), "Father's daughter".to_string()));
        names.insert("Boseda".to_string(), (CharacterType::Farmer, "Yoruba, Nigeria".to_string(), "Born on Sunday".to_string()));
        names.insert("Chiamaka".to_string(), (CharacterType::Farmer, "Igbo, Nigeria".to_string(), "God is beautiful".to_string()));

        // HUNTERS
        names.insert("Folami".to_string(), (CharacterType::Hunter, "Yoruba, Nigeria".to_string(), "Respect and honor me".to_string()));
        names.insert("Gamba".to_string(), (CharacterType::Hunter, "Shona, Zimbabwe".to_string(), "Warrior - Village Guard".to_string()));
        names.insert("Hasani".to_string(), (CharacterType::Hunter, "Swahili, East Africa".to_string(), "Handsome - Scout".to_string()));
        names.insert("Tau".to_string(), (CharacterType::Hunter, "Tswana, Botswana".to_string(), "Lion - Animal trainer".to_string()));

        // CHILDREN
        names.insert("Imara".to_string(), (CharacterType::Child, "Swahili, East Africa".to_string(), "Strong, firm".to_string()));
        names.insert("Jahi".to_string(), (CharacterType::Child, "Swahili, East Africa".to_string(), "Dignity - Young apprentice".to_string()));
        names.insert("Kione".to_string(), (CharacterType::Child, "Swahili, East Africa".to_string(), "Someone who comes from nowhere".to_string()));

        // SPIRITUAL LEADERS
        names.insert("Makena".to_string(), (CharacterType::SpiritualLeader, "Kikuyu, Kenya".to_string(), "The happy one - Priestess".to_string()));
        names.insert("Nuru".to_string(), (CharacterType::SpiritualLeader, "Swahili, East Africa".to_string(), "Light - Spiritual Guide".to_string()));
        names.insert("Oba".to_string(), (CharacterType::SpiritualLeader, "Yoruba, Nigeria".to_string(), "King/Ruler - Chief".to_string()));
        names.insert("Ture".to_string(), (CharacterType::SpiritualLeader, "Central African".to_string(), "Trickster hero - Wandering Sage".to_string()));

        // VILLAGERS
        names.insert("Dumebi".to_string(), (CharacterType::Villager, "Igbo, Nigeria".to_string(), "Follow me - Messenger".to_string()));
        names.insert("Enitan".to_string(), (CharacterType::Villager, "Yoruba, Nigeria".to_string(), "Person of story - Storyteller".to_string()));
        names.insert("Ubuntu".to_string(), (CharacterType::Villager, "Zulu, South Africa".to_string(), "Humanity to others - Mediator".to_string()));
        names.insert("Wanjiru".to_string(), (CharacterType::Villager, "Kikuyu, Kenya".to_string(), "Born of the people - Midwife".to_string()));
        names.insert("Xola".to_string(), (CharacterType::Villager, "Xhosa, South Africa".to_string(), "Stay in peace - Tea house keeper".to_string()));
        names.insert("Yara".to_string(), (CharacterType::Villager, "Arabic/African".to_string(), "Small butterfly - Flower seller".to_string()));
        names.insert("Zalika".to_string(), (CharacterType::Villager, "Swahili, East Africa".to_string(), "Well-born - Noble's daughter".to_string()));

        Self { names }
    }
}

// ============================================================================
// PORTRAIT DATABASE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortraitEmotion {
    Neutral,
    Happy,
    Sad,
    Angry,
    Surprised,
    Determined,
    Sick,
    Waking,
    Laughing,
    Threatening,
    Defeated,
    Enraged,
    Worried,
}

#[derive(Resource)]
pub struct PortraitDB {
    pub portraits: HashMap<(String, PortraitEmotion), Handle<Image>>,
}

impl Default for PortraitDB {
    fn default() -> Self {
        Self {
            portraits: HashMap::new(),
        }
    }
}

impl PortraitDB {
    pub fn get_portrait(&self, character_name: &str, emotion: PortraitEmotion) -> Option<&Handle<Image>> {
        self.portraits.get(&(character_name.to_string(), emotion))
    }

    pub fn add_portrait(&mut self, character_name: String, emotion: PortraitEmotion, handle: Handle<Image>) {
        self.portraits.insert((character_name, emotion), handle);
    }
}
