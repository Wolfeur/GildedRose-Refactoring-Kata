use std::fmt::{self, Display};

pub struct Item {
    pub name: String, // type based on name?
    pub sell_in: i32,
    pub quality: i32, // u8 sufficient?
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>, // need `pub`?
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items { // foreach?
            let item_type = Self::define_type(&item.name);
            let quality_change = Self::define_quality_change(item.sell_in, item.quality, &item_type);
            Self::alter_quality(item, quality_change);
            Self::alter_sell_in(item, &item_type);
        }
    }

    // logic based on educated guesses. specs insufficient
    // method in ItemType instead?
    fn define_type(name: &str) -> ItemType {
        // `else if` unneeded, but I find it more readable.
        if name.starts_with("Aged Brie"){
            return ItemType::AgedBrie;
        } else if name.starts_with("Sulfuras") {
            return ItemType::Sulfuras;
        } else if name.starts_with("Backstage passes") {
            return ItemType::BackstagePasses;
        } else if name.starts_with("Conjured") {
            //what if conjured is doubled with other type? deemed out of scope for now
            return ItemType::Conjured;
        }

        // prefer having a non-conditional return at the end
        return ItemType::Normal;
    }

    fn define_quality_change(sell_in: i32, quality: i32, item_type: &ItemType) -> i32 {
        // exhaustive check
        match item_type {
            ItemType::Sulfuras => 0,
            ItemType::AgedBrie if sell_in <= 0 => 2, //escaped notice at first
            ItemType::AgedBrie => 1,
            ItemType::Conjured if sell_in <= 0 => -4, //unsure if to comprise zero. reflecting other cases
            ItemType::Conjured => -2,
            ItemType::BackstagePasses if sell_in <= 0 => -quality,
            ItemType::BackstagePasses if sell_in <= 5 => 3,
            ItemType::BackstagePasses if sell_in <= 10 => 2,
            ItemType::BackstagePasses => 1,
            ItemType::Normal if sell_in <= 0 => -2,
            ItemType::Normal => -1 //decrement by default. no `_` to ensure exhaustiveness in the future
        }
    }

    fn alter_quality(item: &mut Item, quality_change: i32) {
        if quality_change == 0 { //condition on item type instead?
            return;
        }

        item.quality = (item.quality + quality_change).clamp(MIN_QUALITY, MAX_QUALITY);
    }

    fn alter_sell_in(item: &mut Item, item_type: &ItemType) {
        if item_type == &ItemType::Sulfuras {
            return;
        }

        item.sell_in -= 1;
    }
}

//create item type enum? quality alteration methods in it?
#[derive(PartialEq, Debug)]
enum ItemType {
    Normal,
    AgedBrie,
    Sulfuras,
    BackstagePasses,
    Conjured
}

const MAX_QUALITY: i32 = 50;
const MIN_QUALITY: i32 = 0;

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item, ItemType};

    #[test]
    pub fn test_define_type() {
        let items = vec![
            Item::new("+5 Dexterity Vest", 10, 20),
            Item::new("Aged Brie item", 2, 0),
            Item::new("Sulfuras, Hand of Ragnaros", 0, 80),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20),
            Item::new("Conjured Mana Cake", 3, 6),
        ];
        //let mut rose = GildedRose::new(items);
        //rose.update_quality();
        assert_eq!(GildedRose::define_type(&items[0].name), ItemType::Normal);
        assert_eq!(GildedRose::define_type(&items[1].name), ItemType::AgedBrie);
        assert_eq!(GildedRose::define_type(&items[2].name), ItemType::Sulfuras);
        assert_eq!(GildedRose::define_type(&items[3].name), ItemType::BackstagePasses);
        assert_eq!(GildedRose::define_type(&items[4].name), ItemType::Conjured);
    }

    #[test]
    pub fn test_define_quality_change() {
        let items = vec![
            (0, 0, ItemType::Sulfuras),
            (-1, 0, ItemType::AgedBrie),
            (1, 0, ItemType::AgedBrie),
            (-2, 0, ItemType::Conjured),
            (2, 0, ItemType::Conjured),
            (-1, 40, ItemType::BackstagePasses),
            (5, 0, ItemType::BackstagePasses),
            (10, 0, ItemType::BackstagePasses),
            (11,00, ItemType::BackstagePasses),
            (-1, 0, ItemType::Normal),
            (1, 0, ItemType::Normal),
        ];
        //let mut rose = GildedRose::new(items);
        //rose.update_quality();
        assert_eq!(GildedRose::define_quality_change(items[0].0, items[0].1, &items[0].2), 0);
        assert_eq!(GildedRose::define_quality_change(items[1].0, items[1].1, &items[1].2), 2);
        assert_eq!(GildedRose::define_quality_change(items[2].0, items[2].1, &items[2].2), 1);
        assert_eq!(GildedRose::define_quality_change(items[3].0, items[3].1, &items[3].2), -4);
        assert_eq!(GildedRose::define_quality_change(items[4].0, items[4].1, &items[4].2), -2);
        assert_eq!(GildedRose::define_quality_change(items[5].0, items[5].1, &items[5].2), -40);
        assert_eq!(GildedRose::define_quality_change(items[6].0, items[6].1, &items[6].2), 3);
        assert_eq!(GildedRose::define_quality_change(items[7].0, items[7].1, &items[7].2), 2);
        assert_eq!(GildedRose::define_quality_change(items[8].0, items[8].1, &items[8].2), 1);
        assert_eq!(GildedRose::define_quality_change(items[9].0, items[9].1, &items[9].2), -2);
        assert_eq!(GildedRose::define_quality_change(items[10].0, items[10].1, &items[10].2), -1);
    }

    #[test]
    pub fn test_alter_quality() {
        let mut items = vec![
            Item::new("my item", 10, 40),
            Item::new("my item", 10, 49),
            Item::new("my item", 2, 10),
            Item::new("my item", 2, 1),
            Item::new("my item", 0, 80),
            Item::new("my item", 0, 80),
        ];
        GildedRose::alter_quality(&mut items[0], 2);
        GildedRose::alter_quality(&mut items[1], 2);
        GildedRose::alter_quality(&mut items[2], -2);
        GildedRose::alter_quality(&mut items[3], -2);
        GildedRose::alter_quality(&mut items[4], 0);
        GildedRose::alter_quality(&mut items[5], 1);

        assert_eq!(items[0].quality, 42);
        assert_eq!(items[1].quality, 50);
        assert_eq!(items[2].quality, 8);
        assert_eq!(items[3].quality, 0);
        assert_eq!(items[4].quality, 80);
        assert_eq!(items[5].quality, 50);
    }

    #[test]
    pub fn test_alter_sell_in() {
        let mut items = vec![
            Item::new("my item", 5, 0),
            Item::new("my item", 5, 0),
            Item::new("my item", 5, 0),
            Item::new("my item", 5, 0),
            Item::new("my item", 5, 0),
        ];
        GildedRose::alter_sell_in(&mut items[0], &ItemType::Normal);
        GildedRose::alter_sell_in(&mut items[1], &ItemType::AgedBrie);
        GildedRose::alter_sell_in(&mut items[2], &ItemType::Sulfuras);
        GildedRose::alter_sell_in(&mut items[3], &ItemType::BackstagePasses);
        GildedRose::alter_sell_in(&mut items[4], &ItemType::Conjured);
        assert_eq!(items[0].sell_in, 4);
        assert_eq!(items[1].sell_in, 4);
        assert_eq!(items[2].sell_in, 5);
        assert_eq!(items[3].sell_in, 4);
        assert_eq!(items[4].sell_in, 4);
    }
}
