use std::{error::Error, net::Ipv4Addr, str::FromStr};

pub trait ToUIntIP {
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>>;
}

impl ToUIntIP for u32 {
    #[inline(always)]
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>> {
        Ok(*self)
    }
}

impl ToUIntIP for &str {
    #[inline(always)]
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>> {
        if let Ok(num) = self.parse::<u32>() {
            return Ok(num);
        }
        Ok(u32::from(Ipv4Addr::from_str(self)?))
    }
}

impl ToUIntIP for Ipv4Addr {
    #[inline(always)]
    fn to_u32_ip(&self) -> Result<u32, Box<dyn Error>> {
        Ok(u32::from(*self))
    }
}

#[cfg(test)]
mod test_ip {
    use super::*;

    #[test]
    fn test_ip_str_2_u32() {
        let ip_str = "1.1.1.1";
        let result = ip_str.to_u32_ip().unwrap();
        assert_eq!(result, 1 << 24 | 1 << 16 | 1 << 8 | 1);
    }

    #[test]
    fn test_ip_u32_str() {
        let ip = "12";
        let result = ip.to_u32_ip().unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_ip_u32() {
        let ip: u32 = 33;
        let result = ip.to_u32_ip().unwrap();
        assert_eq!(result, 33);
    }

    #[test]
    fn test_ip_addr() {
        let ip = Ipv4Addr::from_str("0.0.3.12").unwrap();
        let result = ip.to_u32_ip().unwrap();
        assert_eq!(result, 3 << 8 | 12)
    }
}
