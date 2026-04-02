enum MemoryType{
    observation,
    error,
    plan,
    preferences
}
enum ReflectionType{
    pattern,
    principle,
    warning
}
struct Session{
    session_id: Uuid,
    memories:Vec<Memories>,
    reflection: Reflection[],
    project: String,
    created_at:String,
    last_active: String,
    deleted_at:String
}
struct Memories{
    memory_id: Uuid,
    type: MemoryType,
    what: String,
    where:String,
    when: String,
    learned: String,
    reflection: Reflection,
    deleted_at: String
}
struct Reflection {
    reflection_id: Uuid,
    project: String,
    context:String,
    type: ReflectionType,
    importance: u8,
    created_at: String
}
