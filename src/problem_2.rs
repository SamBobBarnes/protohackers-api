use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use downcast_rs::{impl_downcast, Downcast};

enum FinancialRequestType {
    Query,
    Insertion
}

struct FinancialRequestBase {
    a: i32,
    b: i32,
}

trait FinancialRequest: Downcast + Debug {
    fn get_financial_request_type(&self) -> FinancialRequestType;
}

impl_downcast!(FinancialRequest);

struct FinancialQuery {
    base: FinancialRequestBase,
}

impl FinancialQuery {
    pub fn min_time(&self) -> i32 {
        self.base.a
    }
    pub fn max_time(&self) -> i32 {
        self.base.b
    }
}

impl Debug for FinancialQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FinancialQuery {{ min_time: {}, max_time: {} }}", self.min_time(), self.max_time())
    }
}

impl FinancialRequest for FinancialQuery {
    fn get_financial_request_type(&self) -> FinancialRequestType {
        FinancialRequestType::Query
    }
}

struct FinancialInsertion {
    base: FinancialRequestBase,
}

impl FinancialInsertion {
    pub fn timestamp(&self) -> i32 {
        self.base.a
    }
    pub fn price(&self) -> i32 {
        self.base.b
    }
}

impl Debug for FinancialInsertion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FinancialInsertion {{ timestamp: {}, price: {} }}", self.timestamp(), self.price())
    }
}

impl FinancialRequest for FinancialInsertion {
    fn get_financial_request_type(&self) -> FinancialRequestType {
        FinancialRequestType::Insertion
    }
}

impl FinancialRequestBase {
    fn from_bytes(bytes: &[u8]) -> io::Result<Box<dyn FinancialRequest>> {
        if bytes.len() != 9 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid byte length"));
        }

        let request = FinancialRequestBase {
            a: bytes[1..4].iter().fold(0, |acc, &b| (acc << 8) | b as i32),
            b: bytes[4..8].iter().fold(0, |acc, &b| (acc << 8) | b as i32),
        };

        match bytes[0] {
            0x51 => Ok(Box::new(FinancialQuery { base: request })),
            0x49 => Ok(Box::new(FinancialInsertion { base: request })),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid query type")),
        }
    }
}

impl PartialEq for FinancialRequestType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FinancialRequestType::Query, FinancialRequestType::Query) => true,
            (FinancialRequestType::Insertion, FinancialRequestType::Insertion) => true,
            _ => false,
        }
    }
}

fn get_mean_of_data(data:&Vec<(i32,i32)>, min_time:i32, max_time:i32) -> i32 {
    let filtered_data: Vec<&(i32,i32)> = data.iter().filter(|&&(timestamp, _)| timestamp >= min_time && timestamp <= max_time).collect();
    if filtered_data.is_empty() {
        return 0;
    }
    let sum: i64 = filtered_data.iter().map(|&&(_, price)| price as i64).sum();
    let mean = sum / (filtered_data.len() as i64);
    mean as i32
}

pub async fn handle_financial_query(mut stream: TcpStream) -> io::Result<()>  {
    let mut data: Vec<(i32,i32)> = Vec::new();
    let mut buffer = [0; 9];
    println!("------------------");
    loop{
        let n = stream.read_exact(&mut buffer[..]).await?;
        if n == 0 { break; }
        let request = FinancialRequestBase::from_bytes(buffer.as_ref())?;

        if request.get_financial_request_type() == FinancialRequestType::Query {
            let query = request.downcast::<FinancialQuery>().unwrap();
            let mean = get_mean_of_data(&data, query.min_time(), query.max_time());
            stream.write_all(&mean.to_be_bytes()).await?;
        } else {
            let insertion = request.downcast::<FinancialInsertion>().unwrap();
            data.push((insertion.timestamp(), insertion.price()));
            data.sort_by(|a, b| b.0.cmp(&a.0));
        }
    }

    stream.shutdown().await?;
    Ok(())
}