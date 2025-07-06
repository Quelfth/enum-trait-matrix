use syn::{Ident, Item, ItemEnum, ItemMod, ItemTrait, ItemUse, Visibility};


pub struct ParsedModule {
    pub vis: Visibility,
    pub name: Ident,
    pub r#enum: ItemEnum,
    pub r#trait: ItemTrait,
    pub uses: Vec<ItemUse>,
}

impl ParsedModule {
    pub fn parse(item: ItemMod) -> Self {
        let ItemMod { vis, unsafety, ident, content, .. } = item;
        if unsafety.is_some() { panic!("#[enum-trait-matrix] cannot be used on an unsafe module.") }
        let Some((_, content)) = content else { panic!("#[enum-trait-matrix] module must have contents.") };
        
        let mut r#enum: Option<ItemEnum> = None;
        let mut r#trait: Option<ItemTrait> = None;
        
        let mut uses = Vec::new();
        
        for item in content {
            match item {
                Item::Enum(item) => {
                    if r#enum.is_none() {
                        r#enum = Some(item);
                    } else {
                        panic!("Only one enum item is allowed.");
                    }
                }
                Item::Trait(item) => {
                    if r#trait.is_none() {
                        r#trait = Some(item);
                    } else {
                        panic!("Only one trait item is allowed.");
                    }
                }
                Item::Use(item) => {
                    uses.push(item);
                }
                _ => panic!("Invalid item in #[enum-trait-matrix].  Only enum and trait items are allowed.")
            }
        }
        
        let Some(r#enum) = r#enum else {panic!("#[enum-trait-matrix] requires an enum item.")};
        let Some(r#trait) = r#trait else {panic!("#[enum-trait-matrix] requires a trait item.")};
        
        
        
        Self {
            vis,
            name: ident,
            r#enum,
            r#trait,
            uses,
        }
    }
}


