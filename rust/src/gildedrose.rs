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
        for mut item in &mut self.items { // foreach?
            let item_type = Self::define_type(&item.name);
            if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if item.quality > 0 {
                    if item.name != "Sulfuras, Hand of Ragnaros" {
                        item.quality = item.quality - 1;
                    }
                }
            } else { // else if?
                if item.quality < 50 {
                    item.quality = item.quality + 1;

                    if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                        if item.sell_in < 11 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }

                        if item.sell_in < 6 {
                            if item.quality < 50 {
                                item.quality = item.quality + 1;
                            }
                        }
                    }
                }
            }

            if item.name != "Sulfuras, Hand of Ragnaros" {
                item.sell_in = item.sell_in - 1;
            }

            if item.sell_in < 0 {
                if item.name != "Aged Brie" {
                    if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                        if item.quality > 0 {
                            if item.name != "Sulfuras, Hand of Ragnaros" {
                                item.quality = item.quality - 1;
                            }
                        }
                    } else {
                        item.quality = item.quality - item.quality;
                    }
                } else {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
            //lots of logic. private methods would be nice. static method enough?
            //probably better with `match`. enum better? would enforce exhaustiveness.
            //can't define type within items nor in wrapping struct. defined on the fly anyway. maybe not worth it?
        }
    }

    /// logic based on educated guesses
    fn define_type(name: &str) -> ItemType {
        // `else if` unneeded, but I find it more readable.
        if name.starts_with("Aged Brie"){
            return ItemType::AgedBrie;
        } else if name.starts_with("Sulfuras") {
            return ItemType::Sulfuras;
        } else if name.starts_with("Backstage passes") {
            return ItemType::BackstagePasses;
        } else if name.starts_with("Conjured") {
            //what if conjured is doubled with other type? out of scope for now.
            return ItemType::Conjured;
        }

        // prefer having a non-conditional return at the end
        return ItemType::Normal;
    }

    fn define_quality_change(sell_in: i32, quality: i32, item_type: ItemType) -> i32 {
        // exhaustive check
        match item_type {
            ItemType::AgedBrie => 1,
            ItemType::Sulfuras => 0,
            ItemType::Conjured if sell_in < 0 => -4,
            ItemType::Conjured => -2,
            ItemType::BackstagePasses if sell_in < 0 => -quality,
            ItemType::BackstagePasses if sell_in <= 5 => 3,
            ItemType::BackstagePasses if sell_in <= 10 => 2,
            ItemType::BackstagePasses => 1,
            ItemType::Normal if sell_in < 0 => -2,
            _ => -1 //decrement by default
        }
    }

    fn alter_quality(item: &mut Item, quality_change: i32) {
        if (quality_change == 0) { //condition on item type instead?
            return;
        }

        item.quality = (item.quality + quality_change).clamp(MIN_QUALITY, MAX_QUALITY);
    }

    fn alter_sell_in(item: &mut Item, item_type: ItemType) {
        if (item_type == ItemType::Sulfuras) {
            return;
        }

        item.sell_in -= 1;
    }
}

//create item type enum? quality alteration methods in it?
#[derive(PartialEq)]
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
    use super::{GildedRose, Item};

    #[test]
    pub fn foo() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("fixme", rose.items[0].name);
    }
}
