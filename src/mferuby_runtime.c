#include <mruby/compile.h>
#include <mruby/value.h>

mrb_value tmrb_float_value(struct mrb_state *mrb, mrb_float f) {
    return mrb_float_value(mrb, f);
}

mrb_value tmrb_fixnum_value(mrb_int i) {
    return mrb_fixnum_value(i);
}

mrb_value tmrb_nil_value() {
    return mrb_nil_value();
}

mrb_aspec TMRB_ARGS_REQ(uint32_t count) {
    return MRB_ARGS_REQ(count);
}

mrb_aspec TMRB_ARGS_NONE() {
    return MRB_ARGS_NONE();
}

// mrb_state accessor
struct RClass *tmrb_state_object_class(struct mrb_state *mrb) {
    return mrb->object_class;
}

#include <mruby/string.h>
mrb_int TRSTRING_LEN(mrb_value s) {
    return RSTRING_LEN(s);
}

const char *TRSTRING_PTR(mrb_value s) {
    return RSTRING_PTR(s);
}

#include <mruby/array.h>
// Array
mrb_int TRARRAY_LEN(mrb_value array) {
    return RARRAY_LEN(array);
}

#include <mruby/data.h>
// Data

#include <mruby/class.h>
// Class
void TMRB_SET_INSTANCE_TT(struct RClass *class, enum mrb_vtype type) {
    MRB_SET_INSTANCE_TT(class, type);
}
