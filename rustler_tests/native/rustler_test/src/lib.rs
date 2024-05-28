mod test_atom;
mod test_binary;
mod test_codegen;
mod test_dirty;
mod test_env;
mod test_error;
mod test_list;
mod test_local_pid;
mod test_map;
mod test_nif_attrs;
mod test_primitives;
mod test_range;
mod test_resource;
mod test_term;
mod test_thread;
mod test_tuple;

rustler::init!(
    "Elixir.RustlerTest",
    [
        test_primitives::add_u32,
        test_primitives::add_i32,
        test_primitives::echo_u8,
        test_primitives::option_inc,
        test_primitives::erlang_option_inc,
        test_primitives::result_to_int,
        test_primitives::echo_u128,
        test_primitives::echo_i128,
        test_list::sum_list,
        test_list::make_list,
        test_local_pid::compare_local_pids,
        test_local_pid::are_equal_local_pids,
        test_term::term_debug,
        test_term::term_eq,
        test_term::term_cmp,
        test_term::term_internal_hash,
        test_term::term_phash2_hash,
        test_term::term_type,
        test_map::sum_map_values,
        test_map::map_entries,
        test_map::map_entries_reversed,
        test_map::map_from_arrays,
        test_map::map_from_pairs,
        test_map::map_generic,
        test_resource::resource_make,
        test_resource::resource_set_integer_field,
        test_resource::resource_get_integer_field,
        test_resource::resource_make_immutable,
        test_resource::resource_immutable_count,
        test_resource::resource_make_with_binaries,
        test_resource::resource_make_binaries,
        test_atom::atom_to_string,
        test_atom::atom_equals_ok,
        test_atom::binary_to_atom,
        test_atom::binary_to_existing_atom,
        test_binary::make_shorter_subbinary,
        test_binary::parse_integer,
        test_binary::binary_new,
        test_binary::owned_binary_new,
        test_binary::new_binary_new,
        test_binary::unowned_to_owned,
        test_binary::realloc_shrink,
        test_binary::realloc_grow,
        test_binary::encode_string,
        test_binary::decode_iolist,
        test_thread::threaded_fac,
        test_thread::threaded_sleep,
        test_env::send_all,
        test_env::send,
        test_env::whereis_pid,
        test_env::is_process_alive,
        test_env::sublists,
        test_codegen::tuple_echo,
        test_codegen::record_echo,
        test_codegen::map_echo,
        test_codegen::exception_echo,
        test_codegen::struct_echo,
        test_codegen::unit_enum_echo,
        test_codegen::tagged_enum_1_echo,
        test_codegen::tagged_enum_2_echo,
        test_codegen::tagged_enum_3_echo,
        test_codegen::tagged_enum_4_echo,
        test_codegen::untagged_enum_echo,
        test_codegen::untagged_enum_with_truthy,
        test_codegen::untagged_enum_for_issue_370,
        test_codegen::newtype_echo,
        test_codegen::tuplestruct_echo,
        test_codegen::newtype_record_echo,
        test_codegen::tuplestruct_record_echo,
        test_dirty::dirty_cpu,
        test_dirty::dirty_io,
        test_range::sum_range,
        test_error::bad_arg_error,
        test_error::atom_str_error,
        test_error::raise_atom_error,
        test_error::raise_term_with_string_error,
        test_error::raise_term_with_atom_error,
        test_error::term_with_tuple_error,
        test_nif_attrs::can_rename,
        test_tuple::add_from_tuple,
        test_tuple::add_one_to_tuple,
        test_tuple::join_tuple_elements,
        test_tuple::maybe_add_one_to_tuple,
        test_tuple::add_i32_from_tuple,
        test_tuple::greeting_person_from_tuple,
        test_codegen::reserved_keywords::reserved_keywords_type_echo,
        test_codegen::generic_types::generic_struct_echo,
        test_codegen::generic_types::mk_generic_map,
    ],
    load = load
);

fn load(env: rustler::Env, _: rustler::Term) -> bool {
    test_resource::on_load(env);
    true
}
