use std::cell::OnceCell;
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use dpdu_api_types::{ParamByteFieldData, ParamLongFieldData, ParamStructAccessTiming, ParamStructFieldData, ParamStructSessionTiming, PduCpst, PduError, PduObjt, PduPc, PduPt, PDU_ID_UNDEF};
use tracing::warn;
use crate::api::{Api, Result as ApiResult};
use crate::types::PduObjectId;
use crate::utils::{PhantomRef, SendSync, PhantomPtr};

/// With the current design, this structure cannot be created directly. It can only be constructed
/// via the [`from_*`] ethods. This is done to prevent panics when calling [Api::set_com_param()].
///
/// Thus, a [ComParam] is always identified either by an ID or a short name.
#[derive(Clone)]
pub struct PduComParam {
    pub(crate) short_name: OnceCell<String>,

    pub(crate) id: PduObjectId,

    pub class: PduPc,

    pub variant: PduCpVariant
}

impl Hash for PduComParam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for PduComParam {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PduComParam {}

impl PduComParam {
    pub fn from_id(
        id: PduObjectId,
        class: PduPc,
        variant: impl Into<PduCpVariant>
    ) -> Self {
        Self {
            short_name: OnceCell::new(),
            id,
            class,
            variant: variant.into()
        }
    }

    /// Recommended way to construct the current structure.
    pub fn from_short_name(
        api: &Api,
        sn: impl Into<String>,
        class: PduPc,
        variant: impl Into<PduCpVariant>
    ) -> ApiResult<PduComParam> {
        let short_name = sn.into();
        let id = api.pdu_get_object_id(PduObjt::ComParam, &short_name)?;

        let com_param = Self {
            short_name: OnceCell::from(short_name),
            id,
            class,
            variant: variant.into()
        };

        if id == PDU_ID_UNDEF {
            warn!(com_param = com_param.get_debug_name(), "ComParam not supported");
            return Err(PduError::ComParamNotSupported)?;
        }

        Ok(com_param)
    }

    pub fn get_short_name(&self) -> Option<&String> {
        self.short_name.get()
    }

    pub(crate) fn set_short_name(&self, name: impl Into<String>) {
        let _ = self.short_name.set(name.into());
    }

    pub(crate) fn try_init_short_name(&self, api: &Api) -> bool {
        let Some(desc) = api.module_description.as_ref() else {
            return false;
        };

        let opt = Some(desc)
            .and_then(|mdf_desc| mdf_desc.com_params.get_by_id(self.id))
            .and_then(|mdf_cp| mdf_cp.short_name.clone());

        if let Some(short_name) = opt {
            self.set_short_name(short_name);
            return true;
        }

        false
    }

    pub fn get_id(&self) -> &PduObjectId {
        &self.id
    }

    pub(crate) fn get_debug_name(&self) -> String {
        self.short_name
            .get()
            .map(|v| v.clone())
            .unwrap_or_else(|| format!("#{}", self.id))
    }
}

pub type ByteFieldComParam = FieldComParam<u8>;

impl ByteFieldComParam {
    pub fn new(mut data: Vec<u8>, capacity: Option<usize>) -> Self {
        let len = data.len();

        let capacity = capacity
            .and_then(|v| if v > len { Some(v) } else { None })
            .unwrap_or(len);

        data.reserve(capacity - len);

        FieldComParam {
            capacity,
            owned_data: data,
            struct_type: None,
        }
    }

    pub(crate) fn get_pdu_data(&self) -> PhantomRef<'_, ParamByteFieldData> {
        PhantomRef::new(ParamByteFieldData {
            param_max_len: self.owned_data.capacity() as _,
            param_act_len: self.owned_data.len() as _,
            p_data_array: self.owned_data.as_ptr() as _,
        })
    }
}

pub type StructFieldComParam = FieldComParam<StructComParam>;

impl StructFieldComParam {
    pub fn new(struct_type: PduCpst, mut data: Vec<StructComParam>, capacity: Option<usize>) -> Self {
        let len = data.len();

        let capacity = capacity
            .and_then(|v| if v > len { Some(v) } else { None })
            .unwrap_or(len);

        data.reserve(capacity - len);

        FieldComParam {
            capacity: data.len(),
            owned_data: data,
            struct_type: Some(struct_type),
        }
    }

    pub(crate) fn get_pdu_data(&self) -> PhantomRef<'_, ParamStructFieldData> {
        PhantomRef::new(ParamStructFieldData {
            com_param_struct_type: self.struct_type.expect("struct type is set"),
            param_max_entries: self.owned_data.capacity() as _,
            param_act_entries: self.owned_data.len() as _,
            p_struct_array: self.owned_data.as_ptr() as _,
        })
    }
}

pub type LongFieldComParam = FieldComParam<u32>;

impl LongFieldComParam {
    pub fn new(mut data: Vec<u32>, capacity: Option<usize>) -> Self {
        let len = data.len();

        let capacity = capacity
            .and_then(|v| if v > len { Some(v) } else { None })
            .unwrap_or(len);

        data.reserve(capacity - len);

        FieldComParam {
            capacity: data.len(),
            owned_data: data,
            struct_type: None,
        }
    }

    pub(crate) fn get_pdu_data(&self) -> PhantomRef<'_, ParamLongFieldData> {
        PhantomRef::new(ParamLongFieldData {
            param_max_len: self.owned_data.capacity() as _,
            param_act_len: self.owned_data.len() as _,
            p_data_array: self.owned_data.as_ptr() as _,
        })
    }
}

// TODO : impl Debug
#[derive(Clone)]
pub enum PduCpVariant {
    Unum8(u8),
    Snum8(i8),
    Unum16(u16),
    Snum16(i16),
    Unum32(u32),
    Snum32(i32),
    ByteField(ByteFieldComParam),
    StructField(StructFieldComParam),
    LongField(LongFieldComParam),
}

impl From<u8> for PduCpVariant {
    fn from(value: u8) -> Self {
        Self::Unum8(value)
    }
}

impl From<i8> for PduCpVariant {
    fn from(value: i8) -> Self {
        Self::Snum8(value)
    }
}

impl From<u16> for PduCpVariant {
    fn from(value: u16) -> Self {
        Self::Unum16(value)
    }
}

impl From<i16> for PduCpVariant {
    fn from(value: i16) -> Self {
        Self::Snum16(value)
    }
}

impl From<u32> for PduCpVariant {
    fn from(value: u32) -> Self {
        Self::Unum32(value)
    }
}

impl From<i32> for PduCpVariant {
    fn from(value: i32) -> Self {
        Self::Snum32(value)
    }
}

impl From<Vec<u8>> for PduCpVariant {
    fn from(value: Vec<u8>) -> Self {
        Self::ByteField(ByteFieldComParam::new(value, None))
    }
}

impl From<(Vec<u8>, usize)> for PduCpVariant {
    fn from(value: (Vec<u8>, usize)) -> Self {
        Self::ByteField(ByteFieldComParam::new(value.0, Some(value.1)))
    }
}

impl From<Vec<ParamStructAccessTiming>> for PduCpVariant {
    fn from(value: Vec<ParamStructAccessTiming>) -> Self {
        let vec = value.into_iter()
            .map(|v| StructComParam::from(v))
            .collect();

        Self::StructField(StructFieldComParam::new(PduCpst::AccessTiming, vec, None))
    }
}

impl From<(Vec<ParamStructAccessTiming>, usize)> for PduCpVariant {
    fn from(value: (Vec<ParamStructAccessTiming>, usize)) -> Self {
        let vec = value
            .0
            .into_iter()
            .map(|v| StructComParam::from(v))
            .collect();

        Self::StructField(StructFieldComParam::new(PduCpst::AccessTiming, vec, Some(value.1)))
    }
}

impl From<Vec<ParamStructSessionTiming>> for PduCpVariant {
    fn from(value: Vec<ParamStructSessionTiming>) -> Self {
        let vec = value.into_iter()
            .map(|v| StructComParam::from(v))
            .collect();

        Self::StructField(StructFieldComParam::new(PduCpst::SessionTiming, vec, None))
    }
}

impl From<(Vec<ParamStructSessionTiming>, usize)> for PduCpVariant {
    fn from(value: (Vec<ParamStructSessionTiming>, usize)) -> Self {
        let vec = value
            .0
            .into_iter()
            .map(|v| StructComParam::from(v))
            .collect();

        Self::StructField(StructFieldComParam::new(PduCpst::SessionTiming, vec, Some(value.1)))
    }
}

impl From<Vec<u32>> for PduCpVariant {
    fn from(value: Vec<u32>) -> Self {
        Self::LongField(LongFieldComParam::new(value, None))
    }
}

impl From<(Vec<u32>, usize)> for PduCpVariant {
    fn from(value: (Vec<u32>, usize)) -> Self {
        Self::LongField(LongFieldComParam::new(value.0, Some(value.1)))
    }
}

impl PduCpVariant {
    pub fn as_unum8(&self) -> Option<&u8> {
        match self {
            Self::Unum8(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_snum8(&self) -> Option<&i8> {
        match self {
            Self::Snum8(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_unum16(&self) -> Option<&u16> {
        match self {
            Self::Unum16(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_snum16(&self) -> Option<&i16> {
        match self {
            Self::Snum16(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_unum32(&self) -> Option<&u32> {
        match self {
            Self::Unum32(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_snum32(&self) -> Option<&i32> {
        match self {
            Self::Snum32(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_bytefield(&self) -> Option<&ByteFieldComParam> {
        match self {
            Self::ByteField(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_structfield(&self) -> Option<&StructFieldComParam> {
        match self {
            Self::StructField(v) => Some(&v),
            _ => None,
        }
    }

    pub fn as_longfield(&self) -> Option<&LongFieldComParam> {
        match self {
            Self::LongField(v) => Some(&v),
            _ => None,
        }
    }

    pub fn get_pdu_type(&self) -> PduPt {
        match self {
            Self::Unum8(..) => PduPt::Unum8,
            Self::Snum8(..) => PduPt::Snum8,

            Self::Unum16(..) => PduPt::Unum16,
            Self::Snum16(..) => PduPt::Snum16,

            Self::Unum32(..) => PduPt::Snum32,
            Self::Snum32(..) => PduPt::Snum32,

            Self::ByteField(..) => PduPt::ByteField,
            Self::StructField(..) => PduPt::StructField,
            Self::LongField(..) => PduPt::LongField,
        }
    }

    pub(crate) fn get_pdu_ptr(&self) -> PhantomPtr<'_> {
        let ptr: *const c_void = match self {
            Self::Unum8(d) => d as *const u8 as _,
            Self::Snum8(d) => d as *const i8 as _,

            Self::Unum16(d) => d as *const u16 as _,
            Self::Snum16(d) => d as *const i16 as _,

            Self::Unum32(d) => d as *const u32 as _,
            Self::Snum32(d) => d as *const i32 as _,
            
            Self::ByteField(d) => d.get_pdu_data().as_ptr() as _,
            Self::StructField(d) => d.get_pdu_data().as_ptr() as _,
            Self::LongField(d) => d.get_pdu_data().as_ptr() as _,
        };

        PhantomPtr::new(ptr)
    }
}

#[derive(Clone, Default)]
pub struct FieldComParam<T> {
    pub capacity: usize,
    pub owned_data: Vec<T>,
    pub struct_type: Option<PduCpst>,
}

impl<T> Debug for FieldComParam<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FieldComParam");

        s/*.field("pdu_data", &self.pdu_data)*/
            .field("struct_type", &self.struct_type);

        s.finish()
    }
}

#[derive(Copy, Clone)]
pub union StructComParam {
    pub session_timing: ParamStructSessionTiming,
    pub access_timing: ParamStructAccessTiming,
}

impl From<ParamStructAccessTiming> for StructComParam {
    fn from(value: ParamStructAccessTiming) -> Self {
        StructComParam {
            access_timing: value,
        }
    }
}

impl From<ParamStructSessionTiming> for StructComParam {
    fn from(value: ParamStructSessionTiming) -> Self {
        StructComParam {
            session_timing: value,
        }
    }
}