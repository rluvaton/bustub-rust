use crate::types::{DBTypeIdImpl, Value};

struct IntegerTypeHelper {

}

impl IntegerTypeHelper {
    pub(crate) fn add_value(left: &DBTypeIdImpl, right: &DBTypeIdImpl) -> Value {
        todo!()
        // match left {
        //     DBTypeIdImpl::SMALLINT(lhs) => {
        //         match right {
        //             DBTypeIdImpl::SMALLINT(rhs) => {
        //                 return (lhs + rhs).into();
        //             }
        //             DBTypeIdImpl::BIGINT(rhs) => {
        //                 return (lhs + rhs)
        //             }
        //         }
        //     }
        //     DBTypeIdImpl::BIGINT(_) => {}
        // }
        // auto x = left.GetAs<T1>();
        // auto y = right.GetAs<T2>();
        // auto sum1 = static_cast<T1>(x + y);
        // auto sum2 = static_cast<T2>(x + y);
        //
        // if ((x + y) != sum1 && (x + y) != sum2) {
        //     throw Exception(ExceptionType::OUT_OF_RANGE, "Numeric value out of range.");
        // }
        // // Overflow detection
        // if (sizeof(x) >= sizeof(y)) {
        //     if ((x > 0 && y > 0 && sum1 < 0) || (x < 0 && y < 0 && sum1 > 0)) {
        //         throw Exception(ExceptionType::OUT_OF_RANGE, "Numeric value out of range.");
        //     }
        //     return Value(left.GetTypeId(), sum1);
        // }
        // if ((x > 0 && y > 0 && sum2 < 0) || (x < 0 && y < 0 && sum2 > 0)) {
        //     throw Exception(ExceptionType::OUT_OF_RANGE, "Numeric value out of range.");
        // }
        // return Value(right.GetTypeId(), sum2);
    }
}
