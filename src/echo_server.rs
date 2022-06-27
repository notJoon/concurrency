// echo server
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

fn main() {
  // TCP 10000번 포트 리스닝
  let listener =  TcpListener::bind("127.0.0.1:10000").unwrap();

  // 커넥션 리퀘스트 승인
  while let Ok((stream, _)) = listener.accept();

  // 객체 생성
  let stream0 = stream.try_clone().unwrap();
  let mut reader = BufReader::new(stream0);
  let mut writer = BufWriter::new(stream0);

  let mut buf = String::new();
  reader.read_line(&mut buf).unwrap();
  writer.writer(buf.as_bytes()).unwrap();
  writer.flush().unwrap();
}