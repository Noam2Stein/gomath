use quote::quote_spanned;
use syn::{parse_macro_input, Ident};

pub fn self_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    quote_spanned! {
        macro_ident.span() =>
        #macro_ident!(VecNNeg: ElementNeg(vec2_neg, vec3_neg, vec4_neg): Neg(neg));
        #macro_ident!(VecNNot: ElementNot(vec2_not, vec3_not, vec4_not): Not(not));
    }
    .into()
}

pub fn rhs_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    quote_spanned! {
        macro_ident.span() =>
        #macro_ident!(VecNAdd: ElementAdd(vec2_add, vec3_add, vec4_add): Add(add));
        #macro_ident!(VecNSub: ElementSub(vec2_sub, vec3_sub, vec4_sub): Sub(sub));
        #macro_ident!(VecNMul: ElementMul(vec2_mul, vec3_mul, vec4_mul): Mul(mul));
        #macro_ident!(VecNDiv: ElementDiv(vec2_div, vec3_div, vec4_div): Div(div));
        #macro_ident!(VecNRem: ElementRem(vec2_rem, vec3_rem, vec4_rem): Rem(rem));
        #macro_ident!(VecNBitAnd: ElementBitAnd(vec2_bitand, vec3_bitand, vec4_bitand): BitAnd(bitand));
        #macro_ident!(VecNBitOr: ElementBitOr(vec2_bitor, vec3_bitor, vec4_bitor): BitOr(bitor));
        #macro_ident!(VecNBitXor: ElementBitXor(vec2_bitxor, vec3_bitxor, vec4_bitxor): BitXor(bitxor));
        #macro_ident!(VecNShr: ElementShr(vec2_shr, vec3_shr, vec4_shr): Shr(shr));
        #macro_ident!(VecNShl: ElementShl(vec2_shl, vec3_shl, vec4_shl): Shl(shl));
    }
    .into()
}

pub fn assign_ops(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_ident = parse_macro_input!(tokens as Ident);

    quote_spanned! {
        macro_ident.span() =>
        #macro_ident!(VecNAddAssign: ElementAddAssign(vec2_add_assign, vec3_add_assign, vec4_add_assign): AddAssign(add_assign));
        #macro_ident!(VecNSubAssign: ElementSubAssign(vec2_sub_assign, vec3_sub_assign, vec4_sub_assign): SubAssign(sub_assign));
        #macro_ident!(VecNMulAssign: ElementMulAssign(vec2_mul_assign, vec3_mul_assign, vec4_mul_assign): MulAssign(mul_assign));
        #macro_ident!(VecNDivAssign: ElementDivAssign(vec2_div_assign, vec3_div_assign, vec4_div_assign): DivAssign(div_assign));
        #macro_ident!(VecNRemAssign: ElementRemAssign(vec2_rem_assign, vec3_rem_assign, vec4_rem_assign): RemAssign(rem_assign));
        #macro_ident!(VecNBitAndAssign: ElementBitAndAssign(vec2_bitand_assign, vec3_bitand_assign, vec4_bitand_assign): BitAndAssign(bitand_assign));
        #macro_ident!(VecNBitOrAssign: ElementBitOrAssign(vec2_bitor_assign, vec3_bitor_assign, vec4_bitor_assign): BitOrAssign(bitor_assign));
        #macro_ident!(VecNBitXorAssign: ElementBitXorAssign(vec2_bitxor_assign, vec3_bitxor_assign, vec4_bitxor_assign): BitXorAssign(bitxor_assign));
        #macro_ident!(VecNShrAssign: ElementShrAssign(vec2_shr_assign, vec3_shr_assign, vec4_shr_assign): ShrAssign(shr_assign));
        #macro_ident!(VecNShlAssign: ElementShlAssign(vec2_shl_assign, vec3_shl_assign, vec4_shl_assign): ShlAssign(shl_assign));
    }
    .into()
}
