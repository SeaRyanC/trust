//! Type representations
pub type TypeId = u32;

pub enum TsType {
    Any, Unknown, String, Number, Boolean, Void, Null, Undefined, Never,
    Object(ObjectType), Array(Box<TsType>), Union(Vec<TsType>), Function(FnType),
}

pub struct ObjectType { pub properties: Vec<(String, TypeId)> }
pub struct FnType { pub params: Vec<TypeId>, pub ret: TypeId }
