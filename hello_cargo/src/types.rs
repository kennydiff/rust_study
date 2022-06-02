enum Gender{
	Unspecified = 0,
	Male = 1,
	Female =2,
}

enum ConnectionState {
	Init,
	SyncReceived(HalfOpen),
	SyncActSent(HalfOpen),
	AckRecieved(FullSession)
}
struct HalfOpen{}
struct FullSession{}

struct User{
	name: String,
	age: u8,
	gender: Gender,
}						