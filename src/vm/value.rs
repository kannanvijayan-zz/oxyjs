
/**
 * The value-boxing strategy used by oxyjs is called Ex-Boxing, short for exponent boxing.
 *
 * Unlike NaN-Boxing, the NaN bit space of the doubles is not used to represent values, and
 * all doubles are not stored as immediates.
 *
 * Instead, ex-boxing takes advantage of the fact that the most common doubles have exponents
 * from a restricted range.  This restricts the values of the high-bits of the exponents.
 * Doubles within this range are rotated to have the exponent high-bits moved into the
 * boxed-value low-bits.  Doubles falling outside of this range are heap-boxed.
 */

const MASK_TAG: u8 = 0b1111;

const TAG_IMM_I32: u8 = 0b0000;
const TAG_IMM_UNDEF_NULL: u8 = 0b0001;
const TAG_IMM_BOOL: u8 = 0b0010;
const TAG_IMM_STR: u8 = 0b0011;
const TAG_PTR: u8 = 0b0100;


const TAG_IMM_F64_0: u8 = 0b0111;
const TAG_IMM_F64_1: u8 = 0b1000;
const TAG_IMM_F64_2: u8 = 0b1001;
const TAG_IMM_F64_3: u8 = 0b1010;
const TAG_IMM_F64_4: u8 = 0b1011;
const TAG_IMM_F64_5: u8 = 0b1100;
const TAG_IMM_F64_6: u8 = 0b1101;
const TAG_IMM_F64_7: u8 = 0b1110;
const TAG_IMM_F64_8: u8 = 0b1111;
const TAG_IMM_F64_MIN: u8 = 0b0111;
const TAG_IMM_F64_MAX: u8 = 0b1111;

const SHIFT_IMM_I32: u8 = 32;
const SHIFT_IMM_UNDEF_NULL: u8 = 32;
const SHIFT_IMM_BOOL: u8 = 32;

const PAYLOAD_IMM_UNDEF: u32 = 1;
const PAYLOAD_IMM_NULL: u32 = 2;

const SHIFT_IMM_STR_LENGTH: u8 = 4;
const IMM_STR_MAX_LENGTH: u8 = 7;

const F64_TAG_ADJUST: u8 = 0b0111;
const F64_ROTATE: u8 = 5;

/** Opaque heap-value struct. */
pub struct HeapValue;

/** The VM's boxed value format. */
#[derive(Debug, Clone, Copy)]
pub struct Value(u64);

impl Value {
    #[inline(always)]
    fn f64_to_u64(fval: f64) -> u64 {
        unsafe {
            let pf64: *const f64 = &fval;
            let pu64: *const u64 = pf64 as *const u64;
            *pu64
        }
    }
    #[inline(always)]
    fn u64_to_f64(uval: u64) -> f64 {
        unsafe {
            let pu64: *const u64 = &uval;
            let pf64: *const f64 = pu64 as *const f64;
            *pf64
        }
    }

    #[inline(always)]
    fn extract_u64_tag(uval: u64) -> u8 {
        (uval as u8) & MASK_TAG
    }

    #[inline(always)]
    pub fn new_imm_i32(ival: i32) -> Value {
        Value(((ival as u64) << SHIFT_IMM_I32) | (TAG_IMM_I32 as u64))
    }

    #[inline(always)]
    pub fn new_imm_undef() -> Value {
        Value(((PAYLOAD_IMM_UNDEF as u64) << SHIFT_IMM_UNDEF_NULL) | (TAG_IMM_UNDEF_NULL as u64))
    }
    #[inline(always)]
    pub fn new_imm_null() -> Value {
        Value(((PAYLOAD_IMM_NULL as u64) << SHIFT_IMM_UNDEF_NULL) | (TAG_IMM_UNDEF_NULL as u64))
    }

    #[inline(always)]
    pub fn new_imm_bool(bval: bool) -> Value {
        Value(((bval as u64) << SHIFT_IMM_BOOL) | (TAG_IMM_BOOL as u64))
    }

    pub fn new_imm_str(bytes: &[u8]) -> Value {
        let len = bytes.len();
        assert!(bytes.len() <= (IMM_STR_MAX_LENGTH as usize));
        let mut uval = (TAG_IMM_STR as u64) | ((bytes.len() as u64) << SHIFT_IMM_STR_LENGTH);
        for i in 0..bytes.len() {
            uval |= (unsafe { *bytes.get_unchecked(i) } as u64) << ((i+1) * 8);
        }
        Value(uval)
    }

    #[inline(always)]
    pub fn new_ptr(ptr: *const HeapValue) -> Value {
        // Low 4 bits must be zero.
        let uval = ptr as u64;
        assert!(uval & 0xFu64 == 0);
        Value(uval | (TAG_PTR as u64))
    }

    #[inline(always)]
    pub fn new_imm_f64(fval: f64) -> Value {
        let uval = Value::f64_to_u64(fval);
        let rot_uval = uval.rotate_left(F64_ROTATE as u32);
        assert!(Value::extract_u64_tag(rot_uval) <= 0b1000);
        Value(rot_uval + (F64_TAG_ADJUST as u64))
    }

    #[inline(always)]
    fn extract_tag(&self) -> u8 {
        (self.0 as u8) & MASK_TAG
    }
    #[inline(always)]
    pub fn is_imm_i32(&self) -> bool {
        self.extract_tag() == TAG_IMM_I32
    }
    #[inline(always)]
    pub fn is_imm_undef_null(&self) -> bool {
        self.extract_tag() == TAG_IMM_UNDEF_NULL
    }
    #[inline(always)]
    pub fn is_imm_bool(&self) -> bool {
        self.extract_tag() == TAG_IMM_BOOL
    }
    #[inline(always)]
    pub fn is_imm_str(&self) -> bool {
        self.extract_tag() == TAG_IMM_STR
    }
    #[inline(always)]
    pub fn is_ptr(&self) -> bool {
        self.extract_tag() == TAG_PTR
    }
    #[inline(always)]
    pub fn is_imm_f64(&self) -> bool {
        self.extract_tag() >= TAG_IMM_F64_MIN
    }

    #[inline(always)]
    pub fn is_imm_undef(&self) -> bool {
        self.0 == Value::new_imm_undef().0
    }
    #[inline(always)]
    pub fn is_imm_null(&self) -> bool {
        self.0 == Value::new_imm_null().0
    }
    #[inline(always)]
    pub fn is_imm_false(&self) -> bool {
        self.0 == Value::new_imm_bool(false).0
    }
    #[inline(always)]
    pub fn is_imm_true(&self) -> bool {
        self.0 == Value::new_imm_bool(true).0
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_imm_i32(&self) -> i32 {
        assert!(self.is_imm_i32());
        (self.0 >> SHIFT_IMM_I32) as i32
    }
    pub fn get_imm_i32(&self) -> Option<i32> {
        if self.is_imm_i32() {
            Some(unsafe { self.get_unchecked_imm_i32() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_imm_bool(&self) -> bool {
        assert!(self.is_imm_bool());
        (self.0 >> SHIFT_IMM_BOOL) != 0
    }
    pub fn get_imm_bool(&self) -> Option<bool> {
        if self.is_imm_bool() {
            Some(unsafe { self.get_unchecked_imm_bool() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_imm_str_len(&self) -> u8 {
        assert!(self.is_imm_str());
        ((self.0 >> SHIFT_IMM_STR_LENGTH) as u8) & 0x7u8
    }
    #[inline(always)]
    pub unsafe fn get_unchecked_imm_str_char(&self, idx: u8) -> u8 {
        assert!(self.is_imm_str());
        assert!(idx < self.get_unchecked_imm_str_len());
        (self.0 >> ((idx+1)*8)) as u8
    }
    pub unsafe fn get_unchecked_imm_str(&self) -> Vec<u8> {
        assert!(self.is_imm_str());
        let len = self.get_unchecked_imm_str_len();
        let mut bytes = Vec::with_capacity(len as usize);
        for i in 0..len {
            bytes.push(unsafe { self.get_unchecked_imm_str_char(i) });
        }
        bytes
    }
    pub fn get_imm_str(&self) -> Option<Vec<u8>> {
        if self.is_imm_str() {
            Some(unsafe { self.get_unchecked_imm_str() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_ptr(&self) -> *const HeapValue {
        assert!(self.is_ptr());
        self.0 as *const HeapValue
    }
    #[inline(always)]
    pub unsafe fn get_unchecked_mut_ptr(&self) -> *mut HeapValue {
        assert!(self.is_ptr());
        self.0 as *mut HeapValue
    }
    pub fn get_ptr(&self) -> Option<*const HeapValue> {
        if self.is_ptr() {
            Some(unsafe { self.get_unchecked_ptr() })
        } else {
            None
        }
    }
    pub fn get_mut_ptr(&self) -> Option<*mut HeapValue> {
        if self.is_ptr() {
            Some(unsafe { self.get_unchecked_mut_ptr() })
        } else {
            None
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_imm_f64(&self) -> f64 {
        assert!(self.is_imm_f64());
        Value::u64_to_f64((self.0 - (F64_TAG_ADJUST as u64)).rotate_right(F64_ROTATE as u32))
    }
    pub fn get_imm_f64(&self) -> Option<f64> {
        if self.is_imm_f64() {
            Some(unsafe { self.get_unchecked_imm_f64() })
        } else {
            None
        }
    }

    pub fn unpack(&self) -> UnpackedValue {
        match self.extract_tag() {
            TAG_IMM_I32 => {
                UnpackedValue::Int32((self.0 >> SHIFT_IMM_I32) as i32)
            }
            TAG_IMM_UNDEF_NULL => {
                if (self.0 >> SHIFT_IMM_UNDEF_NULL) as u32 == PAYLOAD_IMM_UNDEF {
                    UnpackedValue::Undefined
                } else {
                    UnpackedValue::Null
                }
            }
            TAG_IMM_BOOL => {
                UnpackedValue::Bool((self.0 >> SHIFT_IMM_BOOL) != 0)
            }
            TAG_IMM_STR => {
                UnpackedValue::ImmStr(unsafe { self.get_unchecked_imm_str() })
            }
            TAG_PTR => {
                UnpackedValue::Ptr(unsafe { self.get_unchecked_mut_ptr() })
            }
            TAG_IMM_F64_0 | TAG_IMM_F64_1 | TAG_IMM_F64_2 | TAG_IMM_F64_3 |
            TAG_IMM_F64_4 | TAG_IMM_F64_5 | TAG_IMM_F64_6 | TAG_IMM_F64_7 |
            TAG_IMM_F64_8 => {
                UnpackedValue::Float64(unsafe { self.get_unchecked_imm_f64() })
            }
            _ => { panic!("Unexpected value tag!") }
        }
    }
}

/** An "unpacked" Value enum. */
#[derive(Debug, Clone)]
pub enum UnpackedValue {
    Int32(i32),
    Undefined,
    Null,
    Bool(bool),
    Ptr(*mut HeapValue),
    ImmStr(Vec<u8>),
    Float64(f64)
}
