// loop  : break "값"   가능
// while : break "값"   안됨

fn main()
{
	let mut i = 0;
	let mut sum = 0;

	let value = loop
				{
					i += 1;
					sum += i;
					if i == 10
					{
						//break;
						break sum;  // <= 여긴 ; 필요
									// break 가 return 같은 의미.
									// loop 인경우만 가능, while true 는 안됨
					}
				};

	println!("{:?}", value);
}