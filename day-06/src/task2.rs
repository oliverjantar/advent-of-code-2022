use std::{collections::VecDeque, error::Error};

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = String::from("rnttlvtttmnmpmhpmmzvmmhpmmnrntnnsnrnndvnddmbbtptssjcczmmbwmbmwwmmflfggwzzhjjgppwrwdwqwbbbmwbwgggqccmlmdmgdgqgpqgpgzzndddgdbbsvvfsfppwjwzjjcnjccwrwrprgppbpddnccjggfrggqngqgdqgddvsshqhmqqfvqvlvwvnwwmccrpcczvcczzgsgwwlggqsggmdgglblpblltbbzrbzzcscgccwssbddsmddzzvvhjjwjwrjwrwfftmttplldnllqttdhddmvdmmhsmsmqsqwqpqbpqppqdppbnpnhhppjbbrzrbrggdbddnrdrzrdrpdppzzfzrzgzllwlwzlllwtltslttlmtmqqbbzmzhmmjjwrjrrnzrzhzmmtbbrppmmfddjhdjdsdllrbbpfphpzpwwjvwwdpwpzwwzrrfnfwwfbbpttzjtjptpvvsfvvljjrzzmdmsddpvpgvvdcdwcwhwghwggrjjhdjhddzhhwbhwbwwbrwwrfrmfrfvrfftzzqrzqrqtrqtrqtrqttgctclcwlwwvqqnlnqlnnndmnmggznzhzqzggzrzbbrwbrwrvvscvvmvhmvvbsbjsjfjpfpqpbbbbjtjptpprnndpnnrsswffmnnjhhpqhqwhhbwhbhgbbfrbrjjpmjmzzdzjdjffcvvtwttrrcscvcncqcfcwwpgpfptpcpspllcfcssgbglgdllcdcmcpcqclctcvtvvbhbbdhhczhzshzhbhchzchchlcllwswqqmpmbbqwwzgzhggczgzffsgggjjlcjjpnjjfqjfjvfvrfrlrvvpfpcfcbffnmfmbmbgggplldblddvcvjcvvmbmmmrvrvvnhhqttgdgvdgdqgddcsclljplpbbsvbsbfsffshsgslsllzztggttfccctwtswwrvwvnnfbnnvjnvvlgvvfppmvpmmvpmmcttjffgsffcllncntcncfclcgcppvdvpvrvbrrnnvrvhrvrzvrvnrvrdrnrqrnngsshqhcqqbfqqzmmmzjzqztznttjffqzfqfbqqsggclgclcddtqqdppbjppqvpqvvmqmqnqcnqcqcqvqjqzqbzzbrrgfgddgtgnnpjjbzjjbbtstzszwswggfffmnnpllfvvnwnpwpqwqwlllqlmmzcmcrcmrcmrcrffnrnssjjrdjjwwqgwqgqdggwzwjzzbfbpptctchthbbbqsqggrllldwdffvwwlrwwljwjnncmmjmvjmmqnnmrrgjgvvpqqbppqlpqllqnqdndwwlppcjcdjjcnnmddffgjjrwjrwjjhvhqvhhfssrtsrsgrrwjwhhbqqpzzrdrhrqhrqrprqqmllsszhzllcdllpmpbbmsmjjmwmssvlslwwmtwmmgvgcvggzcgzgddsjsjfsssftfcchhfmfcmcpcpvcppqggjddljjbttwrrhghqggpghgngzngznnzqnnssqlqmqbmmmqdqqhttnqqjhhphrhccbggmjjpwpbwpprdrvvjzvvtctzzmpphwpwdpdvvhbvvdvsdsjdjbdjdljjstswttnhhbfffmnngddlglqggvzvlzvzqqvbqbvbjbnjjsvjjwpjjzshrdtrjttvqnbltrfpvnztrwzrtgjpzgqdjfglqgjrgdzbhqpghdbfhlfjhbfjfjppfgljmgwljlsbmltgztthnzvdrgqlgddvqhzctdgcphfqvnpjjgzwqfvnhvzdrwtpgfdjpqnfshslqmplcprdntnhqqbqptwzvdddhcjcqrfhjqnjvpnhttblwgjwlfwntdchgfjmdbgtqtdgnzbqwzzcltwtmtqtdbvjtfvlzpcvgmrfqwfhwqmhvwhftzgmhshffnjwqbvztszsrrglqvhfpqmbnqjsfnwdwgdtmztbvqrmztfctmvptbwnfzfgdztjgnqsrsqqqnrpgzsqszzwwwgqnnnrdzhzdbqjgbvncprzcjqchzfgnclbrmphbsdwwpvwjwlbshhgjfbhjjtdqrmrcjfnrrhqrpsbglthzpvfglqspttdpwlljhnlrjpzchbrqcgtmcscjnwvpztfjdcwbnbgmbpgdthgnhbrtwftnscbsrndghbslflpcpjwbcjnhzcwdcslmzqbtrlnzmntlpjcsctnsqwtbffqlhfgcsflvfwnmczvsbflnnnzpfjfrcwhhcbtbjcbghtcwcgdbrwrgfgvpwtcwlwcmnmrtcrjbwtwlrfstztsghfvrfjzzpswpqfqpvqstvbhqfjlgmtdlhqrhwzqpnqpllnlgzwptbgftmblqcwfcllbwfzdhrndfrvdvwzqvhnghlzvhldnnvrgqvlpfdnpmcgddjmstzsqfvzwftflrwtzqwjbbqhjpfbdztdfsgsztvvrvslgspgpdcmwszdfsddqhpzpsjgqmgzqvhchlgrmcmzwzbtwfphvgcdmhfdczhffgmqpncdjszzgwfvwsqddvbcgngbjwhmphjsmjthvbthhfwdtqmjctcmdpqpsdrnrzdgzgzctbhwsgvtjgwjbsnnjmpmqgwrnqfqbpnrpddjsrsvmcshhthwfrwmqsrjhlsrgfzvwmdzhwrvchppqldghgzflrnwqnvntmtdwmrpgbdbzvcmnqstzntvllcgzsnvrhqzsfncznhgrggmvrmgsqmhbdsjbsqqhzppfcwdrgdvfjdscrvpwtsdmcnczwbbjhvddprwtzfwslcfdcrqfszcgmhtdfvlqzqtvwngzvmmqcrqpzwzhggjnphsrmnctnfhtppglspnvzrsqfgzdfrrwbzbqwvbvbnzgmdrqrnsvdpvlgcmnggsbmbtfwrvdjrtgtgcqscnfpgswgsngdqnnscffdcnlrcpdpcbpzvqcrtjhlwvgnfhhqmprthrtcvcjjwgprqqdwfbgmzlwttjpvcjzfwbdhvngsjpgtqsvbldbcvhjhzbjzblqtqhlnbzzqfcpnzdhbplztcgvzhbgshqbccgwzhftqvtwzbwmnfrsgphhhgtsmwlqhlcchtbtggqwmbdthhmqqjtfdvfpddfdrtfjbpmwtcbfnrhwcnpdqrdtsfdmjfzdwwgnftnwpssgqtlpdbwhnzcnfmppclsswbhcdghpnslwjznqszgdtrnpncsqsnbrplrfwpbnfnvttlzcjtvhzcpzmhfsfzfjlzqqnprpdvwbfthmrswqrcqqwrnwzmgjqqsnqdblssmhngjjvprmqbswtgzzvprwhrgjqshvmwzgrgfmzlgrtzbmdlzncwqdftfsndvdfmmplswdbjtbcbvcvtpjvrqpghczpqvvpqwfbfhllbpvrrfsmsjhqbldcrwvzvcvzfffqvplbfbdbwctbjsljlfwtbcnpsbtpmcqgdvltmztvrcfsprbnvwplwhncgsdnrdmqnmcvpvvrlmlwtgvrnwvzsfctdlcfvtgqmpnbwcbwvfpmqnbjvwffpjtvvgflhrnlngrzhhttdtdbvscftsqtvbrgzfgsjvwhzjcbsqcttlgwmhhvjhwhgmmdtflfdbvnhgcblqmwjfsqnngjqfbvdnsfbgwjfhpgdgvhpbvlrtpcpvhrbtqpbffzcqrvbqwvqmmrcwtnvcgwvsqzvrwdbcnjshbnbftmmvrmjvgfdwbsjvqfdwnnvqqhbmshcrclrwhfhbtnwqvmrrvdwgcwcsrhdbqndsthmrmbjhttjtzmlflbrmhlcgsbdjcjcvwcjffnqrntpflrgfcngpchtrzpnflwjvcgbwtsnjfqsggwmwhdvbzdpjmtwlmrslnjsndjtgjmmwmdgtnfrztppzqvqhbfzqpsdhvsshddlzwcmsndrpqhndsrjnngnmgmzrvchwlqgdnbssbhpbpgwpsrcnbphpslvqplhpgdhmrnwwjmhvnsfjmrfwtvjjrmgptvjffhbgpmfgmgrcjrwqhccssrqjpljbpwcvsfdtmbhzsmsjsgblgpcqszsttfclrjcnsslmngmbmwqfhddbvmbvwmrmvglsl");
    println!("{}", eval(&data));
    Ok(())
}

fn eval(value: &str) -> usize {
    let mut buf = Buf::new();
    for c in value.chars() {
        if buf.add_char_and_eval(c) {
            break;
        }
    }

    buf.counter
}

struct Buf {
    data: [usize; 26],
    packet: VecDeque<char>,
    counter: usize,
}

impl Buf {
    fn new() -> Self {
        Buf {
            data: [0; 26],
            packet: VecDeque::new(),
            counter: 0,
        }
    }
    fn add_char_and_eval(&mut self, c: char) -> bool {
        self.counter += 1;

        *self.data.get_mut(get_point(&c)).unwrap() += 1;

        self.packet.push_back(c);

        if self.packet.len() < 15 {
            return false;
        }

        if let Some(front) = self.packet.pop_front() {
            let value = self.data.get_mut(get_point(&front)).unwrap();
            *value -= 1;
        }

        for char in self.packet.iter() {
            if *self.data.get(get_point(char)).unwrap() > 1 {
                return false;
            }
        }

        true
    }
}

fn get_point(c: &char) -> usize {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        _ => panic!("invalid char"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for value in data {
            assert_eq!(eval(value.0), value.1);
        }
    }
}
