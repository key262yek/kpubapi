## Purpose
[공공데이터포털](https://www.data.go.kr/index.do)에서 Open API로 제공하는 데이터를 일반적으로 접근해 요청하고, 받은 데이터를 적절한 구조체로 Parsing하는 crate

## Background
처음에는 공공데이터 포털에서 지하철 실시간 도착정보를 얻으려 했다. 하지만 단순히 이 정보만 취급하고 끝내기에 아까워 공공데이터 포털 전체를 아우를 수 있는 API rust wrapper를 만들어보자 결심했다. 

## Requirements
1. Request, Response 형식에 맞춰 선언된 구조체를 적절한 json, xml로 변환하고 역으로 parsing하는 derive macro
2. 데이터 요청 주소, ID 등의 필요 정보를 저장하고 공공데이터 포털과 소통하는 구조체
3. Request trait을 만족하는 변수를 인수로 하는 request 함수

## Detailed design
### Derive macro
구조체의 attribute가 Request의 어떤 변수에 대응되는지 적으면, 이를 토대로 Request trait을 implement 해주는 derive macro.
Optional field인지 여부인지 확인하고, 기본적으로는 attribute의 이름을 그대로 request field로 활용하되, name을 따로 지정해주는 것도 가능하다.
Response에 대해서도 같은 식으로 derive macro를 통해 Response를 parsing할 수 있는 Response trait을 자동으로 implement 해주어야 한다.

Request의 attribute는 각각 json, xml의 문자열로 출력할 수 있어야하므로, `Display` trait이 구현된 구조체여야 할 것이며,
Response의 attribute는 문자열로부터 parsing할 수 있는 `From<String>` trait이 구현되어 있어야 한다.

```rust
use kpubapi::{Request, Response}

#[derive(Request)]
struct RequestData<'a>{
	#[info(name = "comp")]
	comp : Option<&'a str>,
	sdate : Option<Date>,
	itemcls : Category
}

struct Date{
	year : usize,
	month : usize,
	day : usize
}

enum Category {
	Cat1,
	Cat2
}


#[derive(Response)]
struct ResponseData{
	#[info(name = "ITEM_SEQ")]
	seq : usize,
	#[info(name = "TOT_AMT")]
	cost : usize
}
```

## Client
공공데이터포털과 직접적으로 소통하는 Client 구조체가 존재해야 한다.
기본적으로 요청할 api의 주소와 ID 등의 필요정보를 가지고 있으며 Request trait을 가지는 구조체를 인수로 하여 포털로부터 response를 받아 parsing해 결과값을 출력해줄 수 있다. 

```rust
use kpubapi::Client;

fn main(){
    let client = Client::builder()
                    .url("http://localfood.chungnam.go.kr/localfood/openApi01.do")
                    .token("TestID")
                    .build();

    let response = client.send(request);
}
```

## Implementation plan
### Client
- [ ] Builder
- [ ] Send request
- [ ] Parse response

### Derive macro
- [ ] Basic structure for Request, Response trait
- [ ] Detect name of attribute and type
- [ ] Optional attribute
- [ ] Enum attribute 
- [ ] Custom structure attribute
- [ ] Derive `Display` trait for Request
- [ ] Derive `From<String>` trait for Response

## Tests
- [ ] Null request / Connection test
- [ ] Request trait display test
- [ ] Response trait parse test
- [ ] Client send function test
