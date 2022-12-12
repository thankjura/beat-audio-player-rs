#[derive(Debug)]
pub struct TrackRef {
    pub tab_idx: u32,
    pub track_idx: u32,
}

impl TrackRef {
    pub fn new(tab_idx: u32, track_idx: u32) -> Self {
        Self {
            tab_idx,
            track_idx,
        }
    }
}

// impl ToVariant for TrackRef {
//     fn to_variant(&self) -> Variant {
//         (self.tab_idx, self.track_idx).to_variant()
//     }
// }
//
// impl StaticVariantType for TrackRef {
//     fn static_variant_type() -> Cow<'static, VariantTy> {
//         <(u32, u32)>::static_variant_type()
//     }
// }
//
// impl FromVariant for TrackRef {
//     fn from_variant(variant: &Variant) -> Option<Self> {
//         let (tab_idx, track_idx) = variant.get::<(u32, u32)>().expect("The variant needs to be of type (u32, u32).");
//         Some(Self {
//             tab_idx,
//             track_idx,
//         })
//     }
// }