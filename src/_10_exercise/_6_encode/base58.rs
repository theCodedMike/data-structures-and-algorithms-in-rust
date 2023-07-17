/// 转换进制58
const BIG_RADIX: u32 = 58;

/// 前置0用1代替
const ALPHABET_INDEX_0: char = '1';

/// base58编码字符
const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// 进制映射关系  &[u8; 128]
const DIGITS_MAP: &[u8] = &[
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 255, 255,
    255, 255, 255, 255, 255, 9, 10, 11, 12, 13, 14, 15, 16, 255, 17, 18, 19, 20, 21, 255, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31, 32, 255, 255, 255, 255, 255, 255, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 255, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 255, 255, 255,
    255, 255,
];

/// 解码错误类型
#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Invalid,
    InvalidLength,
    InvalidCharacter(char, usize),
}

///编码trait
pub trait Encoder {
    fn encode_to_base58(&self) -> String;
}
///解码trait
pub trait Decoder {
    fn decode_from_base58(&self) -> Result<String, DecodeError>;
}

/// 实现base58编码
impl Encoder for str {
    fn encode_to_base58(&self) -> String {
        // 转换为bytes来处理
        let str_u8 = self.as_bytes();
        let zero_count = str_u8.iter().take_while(|&&x| x == 0).count();
        // 转换后所需空间: log(256)/log(58)约为原数据1.38倍
        // 前置0不需要，所以要减去
        let size = (str_u8.len() - zero_count) * 138 / 100 + 1;
        // 字符进制转换
        let mut i = zero_count;
        let mut high = size - 1;
        let mut buffer = vec![0_u8; size];
        while i < str_u8.len() {
            // j为逐渐减小的下标，对应从后往前
            let mut j = size - 1;
            // carry为从前往后读取的字符
            let mut carry = str_u8[i] as u32;
            // 将转换数据从后往前依次存放
            while j > high || carry != 0 {
                carry += 256 * buffer[j] as u32;
                buffer[j] = (carry % BIG_RADIX) as u8;
                carry /= BIG_RADIX;

                if j > 0 {
                    j -= 1;
                }
            }

            i += 1;
            high = j;
        }

        // 处理多个前置0
        let mut b58_str = String::new();
        for _ in 0..zero_count {
            b58_str.push(ALPHABET_INDEX_0);
        }

        // 获取编码后的字符并拼接成字符串
        let mut j = buffer.iter().take_while(|x| **x == 0).count();
        while j < size {
            b58_str.push(ALPHABET[buffer[j] as usize] as char);
            j += 1;
        }

        b58_str
    }
}

/// 实现base58解码
impl Decoder for str {
    fn decode_from_base58(&self) -> Result<String, DecodeError> {
        // 保存转换字符
        let mut bin = [0_u8; 132];
        let mut out = [0_u32; (132 + 3) / 4];
        // 以4为单元处理数据后剩余的比特数
        let bytes_left = (bin.len() % 4) as u8;
        let zero_mask = match bytes_left {
            0 => 0_u32,
            _ => 0xffffffff << (bytes_left * 8),
        };
        // 统计前置0的个数
        let zero_count = self.chars().take_while(|&x| x == ALPHABET_INDEX_0).count();
        let mut i = zero_count;
        let b58 = self.bytes().collect::<Vec<_>>();
        while i < self.len() {
            // 错误字符
            if b58[i] & 0x80 != 0 {
                return Err(DecodeError::InvalidCharacter(b58[i] as char, i));
            }
            if DIGITS_MAP[b58[i] as usize] == 255 {
                return Err(DecodeError::InvalidCharacter(b58[i] as char, i));
            }
            // 进制转换
            let mut j = out.len();
            let mut c = DIGITS_MAP[b58[i] as usize] as u64;
            while j != 0 {
                j -= 1;
                let t = out[j] as u64 * (BIG_RADIX as u64) + c;
                c = (t & 0x3f00000000) >> 32;
                out[j] = (t & 0xffffffff) as u32;
            }
            // 数据太长
            if c != 0 {
                return Err(DecodeError::InvalidLength);
            }
            if (out[0] & zero_mask) != 0 {
                return Err(DecodeError::InvalidLength);
            }

            i += 1;
        }

        // 处理剩余比特
        let mut i = 1;
        let mut j = 0;
        bin[0] = match bytes_left {
            3 => ((out[0] & 0xff0000) >> 16) as u8,
            2 => (out[0] & 0xff00) as u8,
            1 => {
                j = 1;
                (out[0] & 0xff) as u8
            }
            _ => {
                i = 0;
                bin[0]
            }
        };

        // 以4为处理单元处理数据，通过移位来做除法
        while j < out.len() {
            bin[i] = ((out[j] >> 0x18) & 0xff) as u8;
            bin[i + 1] = ((out[j] >> 0x10) & 0xff) as u8;
            bin[i + 2] = ((out[j] >> 8) & 0xff) as u8;
            bin[i + 3] = ((out[j] >> 0) & 0xff) as u8;
            i += 4;
            j += 1;
        }

        // 获取0的个数
        let leading_zeros = bin.iter().take_while(|&&x| x == 0).count();

        // 获取解码后的字符串
        let new_str = String::from_utf8(bin[leading_zeros - zero_count..].to_vec());
        match new_str {
            Ok(res) => Ok(res),
            Err(_) => Err(DecodeError::Invalid),
        }
    }
}
