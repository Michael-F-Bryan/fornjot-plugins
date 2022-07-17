window.SIDEBAR_ITEMS = {"enum":[["BlockType","The type for a `block`/`if`/`loop`."],["CanonicalOption","Represents options for canonical function definitions."],["ComponentExportKind","Represents the kind of an export from a WebAssembly component."],["ComponentOuterAliasKind","Represents the kinds of outer aliasable items in a component."],["ComponentSectionId","Known section identifiers of WebAssembly components."],["ComponentTypeRef","Represents a reference to a type."],["ComponentValType","Represents a component value type."],["CoreOuterAliasKind","Represents the kinds of outer core aliasable items in a component."],["DataSegmentMode","A data segment’s mode."],["Element","An element in a segment in the element section."],["ElementMode","An element segment’s mode."],["Elements","A sequence of elements in a segment in the element section."],["EntityType","The type of an entity."],["ExportKind","Represents the kind of an export from a WebAssembly module."],["Instruction","WebAssembly instructions."],["ModuleArg","Represents an argument to a module instantiation."],["PrimitiveValType","Represents a primitive component value type."],["SectionId","Known section identifiers of WebAssembly modules."],["TagKind","Represents a tag kind."],["TypeBounds","Represents the possible type bounds for type references."],["ValType","The type of a core WebAssembly value."]],"struct":[["AliasSection","An encoder for the core alias section of WebAssembly components."],["CanonicalFunctionSection","An encoder for the canonical function section of WebAssembly components."],["CodeSection","An encoder for the code section."],["Component","Represents a WebAssembly component that is being encoded."],["ComponentAliasSection","An encoder for the alias section of WebAssembly component."],["ComponentDefinedTypeEncoder","Used for encoding component defined types."],["ComponentExportSection","An encoder for the export section of WebAssembly component."],["ComponentImportSection","An encoder for the import section of WebAssembly components."],["ComponentInstanceSection","An encoder for the instance section of WebAssembly components."],["ComponentStartSection","An encoder for the start section of WebAssembly components."],["ComponentType","Represents a component type."],["ComponentTypeEncoder","Used to encode component and instance types."],["ComponentTypeSection","An encoder for the type section of WebAssembly components."],["CoreTypeEncoder","Used to encode core types."],["CoreTypeSection","An encoder for the core type section of WebAssembly components."],["CustomSection","A custom section holding arbitrary data."],["DataCountSection","An encoder for the data count section."],["DataSection","An encoder for the data section."],["DataSegment","A segment in the data section."],["DataSymbolDefinition","The definition of a data symbol within a symbol table."],["ElementSection","An encoder for the element section."],["ElementSegment","An element segment in the element section."],["ExportSection","An encoder for the export section of WebAssembly module."],["Function","An encoder for a function body within the code section."],["FunctionSection","An encoder for the function section of WebAssembly modules."],["GlobalSection","An encoder for the global section."],["GlobalType","A global’s type."],["ImportSection","An encoder for the import section of WebAssembly modules."],["IndirectNameMap","A map used to describe names with two levels of indirection, as opposed to a [`NameMap`] which has one level of indirection."],["InstanceSection","An encoder for the core instance section of WebAssembly components."],["InstanceType","Represents an instance type."],["LinkingSection","An encoder for the linking custom section."],["MemArg","The immediate for a memory instruction."],["MemorySection","An encoder for the memory section."],["MemoryType","A memory’s type."],["Module","Represents a WebAssembly component that is being encoded."],["ModuleSection","An encoder for the module section of WebAssembly components."],["ModuleType","Represents the type of a core module."],["NameMap","A map used to name items in a wasm module, organized by naming each individual index."],["NameSection","An encoder for the custom `name` section."],["NestedComponentSection","An encoder for the component section of WebAssembly components."],["RawSection","A section made up of uninterpreted, raw bytes."],["StartSection","An encoder for the start section of WebAssembly modules."],["SymbolTable","A subsection of the [linking custom section][crate::LinkingSection] that provides extra information about the symbols present in this Wasm object file."],["TableSection","An encoder for the table section."],["TableType","A table’s type."],["TagSection","An encoder for the tag section."],["TagType","A tag’s type."],["TypeSection","An encoder for the type section of WebAssembly modules."]],"trait":[["ComponentSection","A WebAssembly component section."],["Encode","Implemented by types that can be encoded into a byte sink."],["Section","A WebAssembly module section."]],"type":[["Lane","Describe an unchecked SIMD lane index."]]};