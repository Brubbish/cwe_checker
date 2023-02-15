
```
src
├─ caller
│  ├─ Cargo.toml
│  └─ src
│     └─ main.rs
├─ config.json
├─ cwe_checker_lib
│  ├─ Cargo.toml
│  └─ src
│     ├─ abstract_domain
│     │  ├─ bitvector.rs
│     │  ├─ bricks
│     │  │  ├─ brick.rs
│     │  │  ├─ tests.rs
│     │  │  └─ widening.rs
│     │  ├─ bricks.rs
│     │  ├─ character_inclusion.rs
│     │  ├─ data
│     │  │  ├─ arithmetics.rs
│     │  │  ├─ conditional_specialization.rs
│     │  │  └─ trait_impl.rs
│     │  ├─ data.rs
│     │  ├─ domain_map.rs
│     │  ├─ identifier.rs
│     │  ├─ interval
│     │  │  ├─ bin_ops.rs
│     │  │  ├─ simple_interval
│     │  │  │  └─ tests.rs
│     │  │  ├─ simple_interval.rs
│     │  │  └─ tests.rs
│     │  ├─ interval.rs
│     │  ├─ mem_region
│     │  │  └─ tests.rs
│     │  ├─ mem_region.rs
│     │  ├─ mod.rs
│     │  └─ strings.rs
│     ├─ analysis
│     │  ├─ backward_interprocedural_fixpoint
│     │  │  ├─ mock_context.rs
│     │  │  ├─ mod.rs
│     │  │  └─ tests.rs
│     │  ├─ callgraph.rs
│     │  ├─ dead_variable_elimination
│     │  │  ├─ alive_vars_computation.rs
│     │  │  └─ mod.rs
│     │  ├─ expression_propagation
│     │  │  ├─ mod.rs
│     │  │  └─ tests.rs
│     │  ├─ fixpoint.rs
│     │  ├─ forward_interprocedural_fixpoint.rs
│     │  ├─ function_signature
│     │  │  ├─ access_pattern.rs
│     │  │  ├─ context
│     │  │  │  └─ tests.rs
│     │  │  ├─ context.rs
│     │  │  ├─ global_var_propagation.rs
│     │  │  ├─ mod.rs
│     │  │  ├─ state
│     │  │  │  ├─ call_handling.rs
│     │  │  │  └─ tests.rs
│     │  │  ├─ state.rs
│     │  │  └─ stubs.rs
│     │  ├─ graph.rs
│     │  ├─ interprocedural_fixpoint_generic.rs
│     │  ├─ mod.rs
│     │  ├─ pointer_inference
│     │  │  ├─ context
│     │  │  │  ├─ id_manipulation.rs
│     │  │  │  ├─ mod.rs
│     │  │  │  ├─ stubs.rs
│     │  │  │  ├─ tests.rs
│     │  │  │  └─ trait_impls.rs
│     │  │  ├─ mod.rs
│     │  │  ├─ object
│     │  │  │  ├─ id_manipulation.rs
│     │  │  │  ├─ mod.rs
│     │  │  │  ├─ tests.rs
│     │  │  │  └─ value_access.rs
│     │  │  ├─ object_list
│     │  │  │  ├─ id_manipulation.rs
│     │  │  │  ├─ list_manipulation.rs
│     │  │  │  ├─ mod.rs
│     │  │  │  └─ tests.rs
│     │  │  ├─ state
│     │  │  │  ├─ access_handling.rs
│     │  │  │  ├─ id_manipulation.rs
│     │  │  │  ├─ mod.rs
│     │  │  │  ├─ tests
│     │  │  │  │  ├─ mod.rs
│     │  │  │  │  └─ specialized_expressions.rs
│     │  │  │  └─ value_specialization.rs
│     │  │  ├─ statistics.rs
│     │  │  └─ vsa_result_impl.rs
│     │  ├─ stack_alignment_substitution
│     │  │  ├─ mod.rs
│     │  │  └─ tests.rs
│     │  ├─ string_abstraction
│     │  │  ├─ context
│     │  │  │  ├─ mod.rs
│     │  │  │  ├─ symbol_calls
│     │  │  │  │  ├─ memcpy.rs
│     │  │  │  │  ├─ scanf.rs
│     │  │  │  │  ├─ sprintf
│     │  │  │  │  │  └─ tests.rs
│     │  │  │  │  ├─ sprintf.rs
│     │  │  │  │  ├─ strcat.rs
│     │  │  │  │  └─ tests.rs
│     │  │  │  ├─ symbol_calls.rs
│     │  │  │  ├─ tests.rs
│     │  │  │  ├─ trait_impls
│     │  │  │  │  └─ tests.rs
│     │  │  │  └─ trait_impls.rs
│     │  │  ├─ mod.rs
│     │  │  ├─ state
│     │  │  │  ├─ mod.rs
│     │  │  │  └─ tests.rs
│     │  │  └─ tests.rs
│     │  └─ vsa_results
│     │     └─ mod.rs
│     ├─ checkers
│     │  ├─ cwe_119
│     │  │  ├─ context
│     │  │  │  ├─ bounds_computation.rs
│     │  │  │  ├─ mod.rs
│     │  │  │  ├─ param_replacement.rs
│     │  │  │  ├─ tests.rs
│     │  │  │  └─ trait_impls.rs
│     │  │  ├─ mod.rs
│     │  │  ├─ state.rs
│     │  │  └─ stubs.rs
│     │  ├─ cwe_134.rs
│     │  ├─ cwe_190.rs
│     │  ├─ cwe_215.rs
│     │  ├─ cwe_243.rs
│     │  ├─ cwe_332.rs
│     │  ├─ cwe_367.rs
│     │  ├─ cwe_416
│     │  │  ├─ context.rs
│     │  │  ├─ mod.rs
│     │  │  └─ state.rs
│     │  ├─ cwe_426.rs
│     │  ├─ cwe_467.rs
│     │  ├─ cwe_476
│     │  │  ├─ context.rs
│     │  │  ├─ state.rs
│     │  │  └─ taint.rs
│     │  ├─ cwe_476.rs
│     │  ├─ cwe_560.rs
│     │  ├─ cwe_676.rs
│     │  ├─ cwe_78.rs
│     │  ├─ cwe_782.rs
│     │  └─ cwe_789.rs
│     ├─ checkers.rs
│     ├─ intermediate_representation
│     │  ├─ bitvector.rs
│     │  ├─ blk.rs
│     │  ├─ def.rs
│     │  ├─ expression
│     │  │  ├─ builder.rs
│     │  │  ├─ tests.rs
│     │  │  └─ trivial_operation_substitution.rs
│     │  ├─ expression.rs
│     │  ├─ jmp.rs
│     │  ├─ macros
│     │  │  ├─ mod.rs
│     │  │  └─ tests.rs
│     │  ├─ mod.rs
│     │  ├─ program.rs
│     │  ├─ project
│     │  │  ├─ block_duplication_normalization.rs
│     │  │  └─ propagate_control_flow.rs
│     │  ├─ project.rs
│     │  ├─ runtime_memory_image.rs
│     │  ├─ sub.rs
│     │  ├─ term
│     │  │  └─ builder.rs
│     │  ├─ term.rs
│     │  └─ variable.rs
│     ├─ lib.rs
│     ├─ pcode
│     │  ├─ expressions.rs
│     │  ├─ mod.rs
│     │  ├─ subregister_substitution
│     │  │  ├─ mod.rs
│     │  │  └─ tests.rs
│     │  ├─ term
│     │  │  └─ tests.rs
│     │  └─ term.rs
│     ├─ pipeline
│     │  ├─ mod.rs
│     │  └─ results.rs
│     └─ utils
│        ├─ arguments
│        │  └─ tests.rs
│        ├─ arguments.rs
│        ├─ binary.rs
│        ├─ ghidra.rs
│        ├─ graph_utils.rs
│        ├─ log.rs
│        ├─ mod.rs
│        └─ symbol_utils.rs
├─ ghidra
│  └─ p_code_extractor
│     ├─ bil
│     │  ├─ DatatypeProperties.java
│     │  ├─ ExecutionType.java
│     │  ├─ Expression.java
│     │  ├─ RegisterProperties.java
│     │  └─ Variable.java
│     ├─ internal
│     │  ├─ HelperFunctions.java
│     │  ├─ JumpProcessing.java
│     │  ├─ ParseCspecContent.java
│     │  ├─ PcodeBlockData.java
│     │  ├─ RegisterConvention.java
│     │  └─ TermCreator.java
│     ├─ PcodeExtractor.java
│     ├─ serializer
│     │  └─ Serializer.java
│     ├─ symbol
│     │  ├─ ExternSymbol.java
│     │  └─ ExternSymbolCreator.java
│     └─ term
│        ├─ Arg.java
│        ├─ Blk.java
│        ├─ Call.java
│        ├─ Def.java
│        ├─ Jmp.java
│        ├─ Label.java
│        ├─ Program.java
│        ├─ Project.java
│        ├─ Sub.java
│        ├─ Term.java
│        └─ Tid.java
└─ installer
   ├─ Cargo.toml
   └─ src
      └─ main.rs

```