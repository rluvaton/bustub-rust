

pub(crate) fn is_aggregation_function_name(mut function_name: String) -> bool {
    function_name = function_name.to_lowercase();

    function_name == "min" || function_name == "max" || function_name == "first" || function_name == "last" ||
        function_name == "sum" || function_name == "count" || function_name == "rank" || function_name == "row_number"
}
