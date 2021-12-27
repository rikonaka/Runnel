use std::fmt;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct AuthError;

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Authentication failed!") // user-facing output
    }
}

impl fmt::Debug for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} auth failed}}", file!(), line!()) // programmer-facing output
    }
}

pub struct TcpReadError;

impl fmt::Display for TcpReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tcp read failed!") // user-facing output
    }
}

impl fmt::Debug for TcpReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} read failed}}", file!(), line!()) // programmer-facing output
    }
}

pub struct TcpSendError;

impl fmt::Display for TcpSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tcp send failed!") // user-facing output
    }
}

impl fmt::Debug for TcpSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} send failed}}", file!(), line!()) // programmer-facing output
    }
}