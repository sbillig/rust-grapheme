#[deriving(Eq)]
pub enum CharClass {
    // These are ordered to match the table on [2]
    Other,
    CR,
    LF,
    Control,
    Extend,
    SpacingMark,
    L,
    V,
    T,
    LV,
    LVT,
    RegionalIndicator
}

pub fn char_class(c: char) -> CharClass {
    if is_cr(c) { CR }
    else if is_lf(c) { LF }
    else if is_control(c) { Control }
    else if is_extend(c) { Extend }
    else if is_spacing_mark(c) { SpacingMark }
    else if is_l(c) { L }
    else if is_v(c) { V }
    else if is_t(c) { T }
    else if is_lv(c) { LV }
    else if is_lvt(c) { LVT }
    else if is_regional_indicator(c) { RegionalIndicator }
    else { Other }
}

pub fn is_cr(c: char) -> bool {
    match c {
          '\x0d'
          => true,
        _ => false
    }
}

pub fn is_control(c: char) -> bool {
    match c {
          '\x00' .. '\x09'
        | '\x0b' .. '\x0c'
        | '\x0e' .. '\x1f'
        | '\x7f' .. '\x9f'
        | '\xad'
        | '\u0600' .. '\u0604'
        | '\u06dd'
        | '\u070f'
        | '\u200b'
        | '\u200e' .. '\u200f'
        | '\u2028'
        | '\u2029'
        | '\u202a' .. '\u202e'
        | '\u2060' .. '\u2064'
        | '\u2065' .. '\u2069'
        | '\u206a' .. '\u206f'
        | '\ud800' .. '\udfff'
        | '\ufeff'
        | '\ufff0' .. '\ufff8'
        | '\ufff9' .. '\ufffb'
        | '\U000110bd'
        | '\U0001d173' .. '\U0001d17a'
        | '\U000e0000'
        | '\U000e0001'
        | '\U000e0002' .. '\U000e001f'
        | '\U000e0020' .. '\U000e007f'
        | '\U000e0080' .. '\U000e00ff'
        | '\U000e01f0' .. '\U000e0fff'
          => true,
        _ => false
    }
}

pub fn is_extend(c: char) -> bool {
    match c {
          '\u0300' .. '\u036f'
        | '\u0483' .. '\u0487'
        | '\u0488' .. '\u0489'
        | '\u0591' .. '\u05bd'
        | '\u05bf'
        | '\u05c1' .. '\u05c2'
        | '\u05c4' .. '\u05c5'
        | '\u05c7'
        | '\u0610' .. '\u061a'
        | '\u064b' .. '\u065f'
        | '\u0670'
        | '\u06d6' .. '\u06dc'
        | '\u06df' .. '\u06e4'
        | '\u06e7' .. '\u06e8'
        | '\u06ea' .. '\u06ed'
        | '\u0711'
        | '\u0730' .. '\u074a'
        | '\u07a6' .. '\u07b0'
        | '\u07eb' .. '\u07f3'
        | '\u0816' .. '\u0819'
        | '\u081b' .. '\u0823'
        | '\u0825' .. '\u0827'
        | '\u0829' .. '\u082d'
        | '\u0859' .. '\u085b'
        | '\u08e4' .. '\u08fe'
        | '\u0900' .. '\u0902'
        | '\u093a'
        | '\u093c'
        | '\u0941' .. '\u0948'
        | '\u094d'
        | '\u0951' .. '\u0957'
        | '\u0962' .. '\u0963'
        | '\u0981'
        | '\u09bc'
        | '\u09be'
        | '\u09c1' .. '\u09c4'
        | '\u09cd'
        | '\u09d7'
        | '\u09e2' .. '\u09e3'
        | '\u0a01' .. '\u0a02'
        | '\u0a3c'
        | '\u0a41' .. '\u0a42'
        | '\u0a47' .. '\u0a48'
        | '\u0a4b' .. '\u0a4d'
        | '\u0a51'
        | '\u0a70' .. '\u0a71'
        | '\u0a75'
        | '\u0a81' .. '\u0a82'
        | '\u0abc'
        | '\u0ac1' .. '\u0ac5'
        | '\u0ac7' .. '\u0ac8'
        | '\u0acd'
        | '\u0ae2' .. '\u0ae3'
        | '\u0b01'
        | '\u0b3c'
        | '\u0b3e'
        | '\u0b3f'
        | '\u0b41' .. '\u0b44'
        | '\u0b4d'
        | '\u0b56'
        | '\u0b57'
        | '\u0b62' .. '\u0b63'
        | '\u0b82'
        | '\u0bbe'
        | '\u0bc0'
        | '\u0bcd'
        | '\u0bd7'
        | '\u0c3e' .. '\u0c40'
        | '\u0c46' .. '\u0c48'
        | '\u0c4a' .. '\u0c4d'
        | '\u0c55' .. '\u0c56'
        | '\u0c62' .. '\u0c63'
        | '\u0cbc'
        | '\u0cbf'
        | '\u0cc2'
        | '\u0cc6'
        | '\u0ccc' .. '\u0ccd'
        | '\u0cd5' .. '\u0cd6'
        | '\u0ce2' .. '\u0ce3'
        | '\u0d3e'
        | '\u0d41' .. '\u0d44'
        | '\u0d4d'
        | '\u0d57'
        | '\u0d62' .. '\u0d63'
        | '\u0dca'
        | '\u0dcf'
        | '\u0dd2' .. '\u0dd4'
        | '\u0dd6'
        | '\u0ddf'
        | '\u0e31'
        | '\u0e34' .. '\u0e3a'
        | '\u0e47' .. '\u0e4e'
        | '\u0eb1'
        | '\u0eb4' .. '\u0eb9'
        | '\u0ebb' .. '\u0ebc'
        | '\u0ec8' .. '\u0ecd'
        | '\u0f18' .. '\u0f19'
        | '\u0f35'
        | '\u0f37'
        | '\u0f39'
        | '\u0f71' .. '\u0f7e'
        | '\u0f80' .. '\u0f84'
        | '\u0f86' .. '\u0f87'
        | '\u0f8d' .. '\u0f97'
        | '\u0f99' .. '\u0fbc'
        | '\u0fc6'
        | '\u102d' .. '\u1030'
        | '\u1032' .. '\u1037'
        | '\u1039' .. '\u103a'
        | '\u103d' .. '\u103e'
        | '\u1058' .. '\u1059'
        | '\u105e' .. '\u1060'
        | '\u1071' .. '\u1074'
        | '\u1082'
        | '\u1085' .. '\u1086'
        | '\u108d'
        | '\u109d'
        | '\u135d' .. '\u135f'
        | '\u1712' .. '\u1714'
        | '\u1732' .. '\u1734'
        | '\u1752' .. '\u1753'
        | '\u1772' .. '\u1773'
        | '\u17b4' .. '\u17b5'
        | '\u17b7' .. '\u17bd'
        | '\u17c6'
        | '\u17c9' .. '\u17d3'
        | '\u17dd'
        | '\u180b' .. '\u180d'
        | '\u18a9'
        | '\u1920' .. '\u1922'
        | '\u1927' .. '\u1928'
        | '\u1932'
        | '\u1939' .. '\u193b'
        | '\u1a17' .. '\u1a18'
        | '\u1a56'
        | '\u1a58' .. '\u1a5e'
        | '\u1a60'
        | '\u1a62'
        | '\u1a65' .. '\u1a6c'
        | '\u1a73' .. '\u1a7c'
        | '\u1a7f'
        | '\u1b00' .. '\u1b03'
        | '\u1b34'
        | '\u1b36' .. '\u1b3a'
        | '\u1b3c'
        | '\u1b42'
        | '\u1b6b' .. '\u1b73'
        | '\u1b80' .. '\u1b81'
        | '\u1ba2' .. '\u1ba5'
        | '\u1ba8' .. '\u1ba9'
        | '\u1bab'
        | '\u1be6'
        | '\u1be8' .. '\u1be9'
        | '\u1bed'
        | '\u1bef' .. '\u1bf1'
        | '\u1c2c' .. '\u1c33'
        | '\u1c36' .. '\u1c37'
        | '\u1cd0' .. '\u1cd2'
        | '\u1cd4' .. '\u1ce0'
        | '\u1ce2' .. '\u1ce8'
        | '\u1ced'
        | '\u1cf4'
        | '\u1dc0' .. '\u1de6'
        | '\u1dfc' .. '\u1dff'
        | '\u200c' .. '\u200d'
        | '\u20d0' .. '\u20dc'
        | '\u20dd' .. '\u20e0'
        | '\u20e1'
        | '\u20e2' .. '\u20e4'
        | '\u20e5' .. '\u20f0'
        | '\u2cef' .. '\u2cf1'
        | '\u2d7f'
        | '\u2de0' .. '\u2dff'
        | '\u302a' .. '\u302d'
        | '\u302e' .. '\u302f'
        | '\u3099' .. '\u309a'
        | '\ua66f'
        | '\ua670' .. '\ua672'
        | '\ua674' .. '\ua67d'
        | '\ua69f'
        | '\ua6f0' .. '\ua6f1'
        | '\ua802'
        | '\ua806'
        | '\ua80b'
        | '\ua825' .. '\ua826'
        | '\ua8c4'
        | '\ua8e0' .. '\ua8f1'
        | '\ua926' .. '\ua92d'
        | '\ua947' .. '\ua951'
        | '\ua980' .. '\ua982'
        | '\ua9b3'
        | '\ua9b6' .. '\ua9b9'
        | '\ua9bc'
        | '\uaa29' .. '\uaa2e'
        | '\uaa31' .. '\uaa32'
        | '\uaa35' .. '\uaa36'
        | '\uaa43'
        | '\uaa4c'
        | '\uaab0'
        | '\uaab2' .. '\uaab4'
        | '\uaab7' .. '\uaab8'
        | '\uaabe' .. '\uaabf'
        | '\uaac1'
        | '\uaaec' .. '\uaaed'
        | '\uaaf6'
        | '\uabe5'
        | '\uabe8'
        | '\uabed'
        | '\ufb1e'
        | '\ufe00' .. '\ufe0f'
        | '\ufe20' .. '\ufe26'
        | '\uff9e' .. '\uff9f'
        | '\U000101fd'
        | '\U00010a01' .. '\U00010a03'
        | '\U00010a05' .. '\U00010a06'
        | '\U00010a0c' .. '\U00010a0f'
        | '\U00010a38' .. '\U00010a3a'
        | '\U00010a3f'
        | '\U00011001'
        | '\U00011038' .. '\U00011046'
        | '\U00011080' .. '\U00011081'
        | '\U000110b3' .. '\U000110b6'
        | '\U000110b9' .. '\U000110ba'
        | '\U00011100' .. '\U00011102'
        | '\U00011127' .. '\U0001112b'
        | '\U0001112d' .. '\U00011134'
        | '\U00011180' .. '\U00011181'
        | '\U000111b6' .. '\U000111be'
        | '\U000116ab'
        | '\U000116ad'
        | '\U000116b0' .. '\U000116b5'
        | '\U000116b7'
        | '\U00016f8f' .. '\U00016f92'
        | '\U0001d165'
        | '\U0001d167' .. '\U0001d169'
        | '\U0001d16e' .. '\U0001d172'
        | '\U0001d17b' .. '\U0001d182'
        | '\U0001d185' .. '\U0001d18b'
        | '\U0001d1aa' .. '\U0001d1ad'
        | '\U0001d242' .. '\U0001d244'
        | '\U000e0100' .. '\U000e01ef'
          => true,
        _ => false
    }
}

pub fn is_l(c: char) -> bool {
    match c {
          '\u1100' .. '\u115f'
        | '\ua960' .. '\ua97c'
          => true,
        _ => false
    }
}

pub fn is_lf(c: char) -> bool {
    match c {
          '\x0a'
          => true,
        _ => false
    }
}

pub fn is_lv(c: char) -> bool {
    match c {
          '\uac00'
        | '\uac1c'
        | '\uac38'
        | '\uac54'
        | '\uac70'
        | '\uac8c'
        | '\uaca8'
        | '\uacc4'
        | '\uace0'
        | '\uacfc'
        | '\uad18'
        | '\uad34'
        | '\uad50'
        | '\uad6c'
        | '\uad88'
        | '\uada4'
        | '\uadc0'
        | '\uaddc'
        | '\uadf8'
        | '\uae14'
        | '\uae30'
        | '\uae4c'
        | '\uae68'
        | '\uae84'
        | '\uaea0'
        | '\uaebc'
        | '\uaed8'
        | '\uaef4'
        | '\uaf10'
        | '\uaf2c'
        | '\uaf48'
        | '\uaf64'
        | '\uaf80'
        | '\uaf9c'
        | '\uafb8'
        | '\uafd4'
        | '\uaff0'
        | '\ub00c'
        | '\ub028'
        | '\ub044'
        | '\ub060'
        | '\ub07c'
        | '\ub098'
        | '\ub0b4'
        | '\ub0d0'
        | '\ub0ec'
        | '\ub108'
        | '\ub124'
        | '\ub140'
        | '\ub15c'
        | '\ub178'
        | '\ub194'
        | '\ub1b0'
        | '\ub1cc'
        | '\ub1e8'
        | '\ub204'
        | '\ub220'
        | '\ub23c'
        | '\ub258'
        | '\ub274'
        | '\ub290'
        | '\ub2ac'
        | '\ub2c8'
        | '\ub2e4'
        | '\ub300'
        | '\ub31c'
        | '\ub338'
        | '\ub354'
        | '\ub370'
        | '\ub38c'
        | '\ub3a8'
        | '\ub3c4'
        | '\ub3e0'
        | '\ub3fc'
        | '\ub418'
        | '\ub434'
        | '\ub450'
        | '\ub46c'
        | '\ub488'
        | '\ub4a4'
        | '\ub4c0'
        | '\ub4dc'
        | '\ub4f8'
        | '\ub514'
        | '\ub530'
        | '\ub54c'
        | '\ub568'
        | '\ub584'
        | '\ub5a0'
        | '\ub5bc'
        | '\ub5d8'
        | '\ub5f4'
        | '\ub610'
        | '\ub62c'
        | '\ub648'
        | '\ub664'
        | '\ub680'
        | '\ub69c'
        | '\ub6b8'
        | '\ub6d4'
        | '\ub6f0'
        | '\ub70c'
        | '\ub728'
        | '\ub744'
        | '\ub760'
        | '\ub77c'
        | '\ub798'
        | '\ub7b4'
        | '\ub7d0'
        | '\ub7ec'
        | '\ub808'
        | '\ub824'
        | '\ub840'
        | '\ub85c'
        | '\ub878'
        | '\ub894'
        | '\ub8b0'
        | '\ub8cc'
        | '\ub8e8'
        | '\ub904'
        | '\ub920'
        | '\ub93c'
        | '\ub958'
        | '\ub974'
        | '\ub990'
        | '\ub9ac'
        | '\ub9c8'
        | '\ub9e4'
        | '\uba00'
        | '\uba1c'
        | '\uba38'
        | '\uba54'
        | '\uba70'
        | '\uba8c'
        | '\ubaa8'
        | '\ubac4'
        | '\ubae0'
        | '\ubafc'
        | '\ubb18'
        | '\ubb34'
        | '\ubb50'
        | '\ubb6c'
        | '\ubb88'
        | '\ubba4'
        | '\ubbc0'
        | '\ubbdc'
        | '\ubbf8'
        | '\ubc14'
        | '\ubc30'
        | '\ubc4c'
        | '\ubc68'
        | '\ubc84'
        | '\ubca0'
        | '\ubcbc'
        | '\ubcd8'
        | '\ubcf4'
        | '\ubd10'
        | '\ubd2c'
        | '\ubd48'
        | '\ubd64'
        | '\ubd80'
        | '\ubd9c'
        | '\ubdb8'
        | '\ubdd4'
        | '\ubdf0'
        | '\ube0c'
        | '\ube28'
        | '\ube44'
        | '\ube60'
        | '\ube7c'
        | '\ube98'
        | '\ubeb4'
        | '\ubed0'
        | '\ubeec'
        | '\ubf08'
        | '\ubf24'
        | '\ubf40'
        | '\ubf5c'
        | '\ubf78'
        | '\ubf94'
        | '\ubfb0'
        | '\ubfcc'
        | '\ubfe8'
        | '\uc004'
        | '\uc020'
        | '\uc03c'
        | '\uc058'
        | '\uc074'
        | '\uc090'
        | '\uc0ac'
        | '\uc0c8'
        | '\uc0e4'
        | '\uc100'
        | '\uc11c'
        | '\uc138'
        | '\uc154'
        | '\uc170'
        | '\uc18c'
        | '\uc1a8'
        | '\uc1c4'
        | '\uc1e0'
        | '\uc1fc'
        | '\uc218'
        | '\uc234'
        | '\uc250'
        | '\uc26c'
        | '\uc288'
        | '\uc2a4'
        | '\uc2c0'
        | '\uc2dc'
        | '\uc2f8'
        | '\uc314'
        | '\uc330'
        | '\uc34c'
        | '\uc368'
        | '\uc384'
        | '\uc3a0'
        | '\uc3bc'
        | '\uc3d8'
        | '\uc3f4'
        | '\uc410'
        | '\uc42c'
        | '\uc448'
        | '\uc464'
        | '\uc480'
        | '\uc49c'
        | '\uc4b8'
        | '\uc4d4'
        | '\uc4f0'
        | '\uc50c'
        | '\uc528'
        | '\uc544'
        | '\uc560'
        | '\uc57c'
        | '\uc598'
        | '\uc5b4'
        | '\uc5d0'
        | '\uc5ec'
        | '\uc608'
        | '\uc624'
        | '\uc640'
        | '\uc65c'
        | '\uc678'
        | '\uc694'
        | '\uc6b0'
        | '\uc6cc'
        | '\uc6e8'
        | '\uc704'
        | '\uc720'
        | '\uc73c'
        | '\uc758'
        | '\uc774'
        | '\uc790'
        | '\uc7ac'
        | '\uc7c8'
        | '\uc7e4'
        | '\uc800'
        | '\uc81c'
        | '\uc838'
        | '\uc854'
        | '\uc870'
        | '\uc88c'
        | '\uc8a8'
        | '\uc8c4'
        | '\uc8e0'
        | '\uc8fc'
        | '\uc918'
        | '\uc934'
        | '\uc950'
        | '\uc96c'
        | '\uc988'
        | '\uc9a4'
        | '\uc9c0'
        | '\uc9dc'
        | '\uc9f8'
        | '\uca14'
        | '\uca30'
        | '\uca4c'
        | '\uca68'
        | '\uca84'
        | '\ucaa0'
        | '\ucabc'
        | '\ucad8'
        | '\ucaf4'
        | '\ucb10'
        | '\ucb2c'
        | '\ucb48'
        | '\ucb64'
        | '\ucb80'
        | '\ucb9c'
        | '\ucbb8'
        | '\ucbd4'
        | '\ucbf0'
        | '\ucc0c'
        | '\ucc28'
        | '\ucc44'
        | '\ucc60'
        | '\ucc7c'
        | '\ucc98'
        | '\uccb4'
        | '\uccd0'
        | '\uccec'
        | '\ucd08'
        | '\ucd24'
        | '\ucd40'
        | '\ucd5c'
        | '\ucd78'
        | '\ucd94'
        | '\ucdb0'
        | '\ucdcc'
        | '\ucde8'
        | '\uce04'
        | '\uce20'
        | '\uce3c'
        | '\uce58'
        | '\uce74'
        | '\uce90'
        | '\uceac'
        | '\ucec8'
        | '\ucee4'
        | '\ucf00'
        | '\ucf1c'
        | '\ucf38'
        | '\ucf54'
        | '\ucf70'
        | '\ucf8c'
        | '\ucfa8'
        | '\ucfc4'
        | '\ucfe0'
        | '\ucffc'
        | '\ud018'
        | '\ud034'
        | '\ud050'
        | '\ud06c'
        | '\ud088'
        | '\ud0a4'
        | '\ud0c0'
        | '\ud0dc'
        | '\ud0f8'
        | '\ud114'
        | '\ud130'
        | '\ud14c'
        | '\ud168'
        | '\ud184'
        | '\ud1a0'
        | '\ud1bc'
        | '\ud1d8'
        | '\ud1f4'
        | '\ud210'
        | '\ud22c'
        | '\ud248'
        | '\ud264'
        | '\ud280'
        | '\ud29c'
        | '\ud2b8'
        | '\ud2d4'
        | '\ud2f0'
        | '\ud30c'
        | '\ud328'
        | '\ud344'
        | '\ud360'
        | '\ud37c'
        | '\ud398'
        | '\ud3b4'
        | '\ud3d0'
        | '\ud3ec'
        | '\ud408'
        | '\ud424'
        | '\ud440'
        | '\ud45c'
        | '\ud478'
        | '\ud494'
        | '\ud4b0'
        | '\ud4cc'
        | '\ud4e8'
        | '\ud504'
        | '\ud520'
        | '\ud53c'
        | '\ud558'
        | '\ud574'
        | '\ud590'
        | '\ud5ac'
        | '\ud5c8'
        | '\ud5e4'
        | '\ud600'
        | '\ud61c'
        | '\ud638'
        | '\ud654'
        | '\ud670'
        | '\ud68c'
        | '\ud6a8'
        | '\ud6c4'
        | '\ud6e0'
        | '\ud6fc'
        | '\ud718'
        | '\ud734'
        | '\ud750'
        | '\ud76c'
        | '\ud788'
          => true,
        _ => false
    }
}

pub fn is_lvt(c: char) -> bool {
    match c {
          '\uac01' .. '\uac1b'
        | '\uac1d' .. '\uac37'
        | '\uac39' .. '\uac53'
        | '\uac55' .. '\uac6f'
        | '\uac71' .. '\uac8b'
        | '\uac8d' .. '\uaca7'
        | '\uaca9' .. '\uacc3'
        | '\uacc5' .. '\uacdf'
        | '\uace1' .. '\uacfb'
        | '\uacfd' .. '\uad17'
        | '\uad19' .. '\uad33'
        | '\uad35' .. '\uad4f'
        | '\uad51' .. '\uad6b'
        | '\uad6d' .. '\uad87'
        | '\uad89' .. '\uada3'
        | '\uada5' .. '\uadbf'
        | '\uadc1' .. '\uaddb'
        | '\uaddd' .. '\uadf7'
        | '\uadf9' .. '\uae13'
        | '\uae15' .. '\uae2f'
        | '\uae31' .. '\uae4b'
        | '\uae4d' .. '\uae67'
        | '\uae69' .. '\uae83'
        | '\uae85' .. '\uae9f'
        | '\uaea1' .. '\uaebb'
        | '\uaebd' .. '\uaed7'
        | '\uaed9' .. '\uaef3'
        | '\uaef5' .. '\uaf0f'
        | '\uaf11' .. '\uaf2b'
        | '\uaf2d' .. '\uaf47'
        | '\uaf49' .. '\uaf63'
        | '\uaf65' .. '\uaf7f'
        | '\uaf81' .. '\uaf9b'
        | '\uaf9d' .. '\uafb7'
        | '\uafb9' .. '\uafd3'
        | '\uafd5' .. '\uafef'
        | '\uaff1' .. '\ub00b'
        | '\ub00d' .. '\ub027'
        | '\ub029' .. '\ub043'
        | '\ub045' .. '\ub05f'
        | '\ub061' .. '\ub07b'
        | '\ub07d' .. '\ub097'
        | '\ub099' .. '\ub0b3'
        | '\ub0b5' .. '\ub0cf'
        | '\ub0d1' .. '\ub0eb'
        | '\ub0ed' .. '\ub107'
        | '\ub109' .. '\ub123'
        | '\ub125' .. '\ub13f'
        | '\ub141' .. '\ub15b'
        | '\ub15d' .. '\ub177'
        | '\ub179' .. '\ub193'
        | '\ub195' .. '\ub1af'
        | '\ub1b1' .. '\ub1cb'
        | '\ub1cd' .. '\ub1e7'
        | '\ub1e9' .. '\ub203'
        | '\ub205' .. '\ub21f'
        | '\ub221' .. '\ub23b'
        | '\ub23d' .. '\ub257'
        | '\ub259' .. '\ub273'
        | '\ub275' .. '\ub28f'
        | '\ub291' .. '\ub2ab'
        | '\ub2ad' .. '\ub2c7'
        | '\ub2c9' .. '\ub2e3'
        | '\ub2e5' .. '\ub2ff'
        | '\ub301' .. '\ub31b'
        | '\ub31d' .. '\ub337'
        | '\ub339' .. '\ub353'
        | '\ub355' .. '\ub36f'
        | '\ub371' .. '\ub38b'
        | '\ub38d' .. '\ub3a7'
        | '\ub3a9' .. '\ub3c3'
        | '\ub3c5' .. '\ub3df'
        | '\ub3e1' .. '\ub3fb'
        | '\ub3fd' .. '\ub417'
        | '\ub419' .. '\ub433'
        | '\ub435' .. '\ub44f'
        | '\ub451' .. '\ub46b'
        | '\ub46d' .. '\ub487'
        | '\ub489' .. '\ub4a3'
        | '\ub4a5' .. '\ub4bf'
        | '\ub4c1' .. '\ub4db'
        | '\ub4dd' .. '\ub4f7'
        | '\ub4f9' .. '\ub513'
        | '\ub515' .. '\ub52f'
        | '\ub531' .. '\ub54b'
        | '\ub54d' .. '\ub567'
        | '\ub569' .. '\ub583'
        | '\ub585' .. '\ub59f'
        | '\ub5a1' .. '\ub5bb'
        | '\ub5bd' .. '\ub5d7'
        | '\ub5d9' .. '\ub5f3'
        | '\ub5f5' .. '\ub60f'
        | '\ub611' .. '\ub62b'
        | '\ub62d' .. '\ub647'
        | '\ub649' .. '\ub663'
        | '\ub665' .. '\ub67f'
        | '\ub681' .. '\ub69b'
        | '\ub69d' .. '\ub6b7'
        | '\ub6b9' .. '\ub6d3'
        | '\ub6d5' .. '\ub6ef'
        | '\ub6f1' .. '\ub70b'
        | '\ub70d' .. '\ub727'
        | '\ub729' .. '\ub743'
        | '\ub745' .. '\ub75f'
        | '\ub761' .. '\ub77b'
        | '\ub77d' .. '\ub797'
        | '\ub799' .. '\ub7b3'
        | '\ub7b5' .. '\ub7cf'
        | '\ub7d1' .. '\ub7eb'
        | '\ub7ed' .. '\ub807'
        | '\ub809' .. '\ub823'
        | '\ub825' .. '\ub83f'
        | '\ub841' .. '\ub85b'
        | '\ub85d' .. '\ub877'
        | '\ub879' .. '\ub893'
        | '\ub895' .. '\ub8af'
        | '\ub8b1' .. '\ub8cb'
        | '\ub8cd' .. '\ub8e7'
        | '\ub8e9' .. '\ub903'
        | '\ub905' .. '\ub91f'
        | '\ub921' .. '\ub93b'
        | '\ub93d' .. '\ub957'
        | '\ub959' .. '\ub973'
        | '\ub975' .. '\ub98f'
        | '\ub991' .. '\ub9ab'
        | '\ub9ad' .. '\ub9c7'
        | '\ub9c9' .. '\ub9e3'
        | '\ub9e5' .. '\ub9ff'
        | '\uba01' .. '\uba1b'
        | '\uba1d' .. '\uba37'
        | '\uba39' .. '\uba53'
        | '\uba55' .. '\uba6f'
        | '\uba71' .. '\uba8b'
        | '\uba8d' .. '\ubaa7'
        | '\ubaa9' .. '\ubac3'
        | '\ubac5' .. '\ubadf'
        | '\ubae1' .. '\ubafb'
        | '\ubafd' .. '\ubb17'
        | '\ubb19' .. '\ubb33'
        | '\ubb35' .. '\ubb4f'
        | '\ubb51' .. '\ubb6b'
        | '\ubb6d' .. '\ubb87'
        | '\ubb89' .. '\ubba3'
        | '\ubba5' .. '\ubbbf'
        | '\ubbc1' .. '\ubbdb'
        | '\ubbdd' .. '\ubbf7'
        | '\ubbf9' .. '\ubc13'
        | '\ubc15' .. '\ubc2f'
        | '\ubc31' .. '\ubc4b'
        | '\ubc4d' .. '\ubc67'
        | '\ubc69' .. '\ubc83'
        | '\ubc85' .. '\ubc9f'
        | '\ubca1' .. '\ubcbb'
        | '\ubcbd' .. '\ubcd7'
        | '\ubcd9' .. '\ubcf3'
        | '\ubcf5' .. '\ubd0f'
        | '\ubd11' .. '\ubd2b'
        | '\ubd2d' .. '\ubd47'
        | '\ubd49' .. '\ubd63'
        | '\ubd65' .. '\ubd7f'
        | '\ubd81' .. '\ubd9b'
        | '\ubd9d' .. '\ubdb7'
        | '\ubdb9' .. '\ubdd3'
        | '\ubdd5' .. '\ubdef'
        | '\ubdf1' .. '\ube0b'
        | '\ube0d' .. '\ube27'
        | '\ube29' .. '\ube43'
        | '\ube45' .. '\ube5f'
        | '\ube61' .. '\ube7b'
        | '\ube7d' .. '\ube97'
        | '\ube99' .. '\ubeb3'
        | '\ubeb5' .. '\ubecf'
        | '\ubed1' .. '\ubeeb'
        | '\ubeed' .. '\ubf07'
        | '\ubf09' .. '\ubf23'
        | '\ubf25' .. '\ubf3f'
        | '\ubf41' .. '\ubf5b'
        | '\ubf5d' .. '\ubf77'
        | '\ubf79' .. '\ubf93'
        | '\ubf95' .. '\ubfaf'
        | '\ubfb1' .. '\ubfcb'
        | '\ubfcd' .. '\ubfe7'
        | '\ubfe9' .. '\uc003'
        | '\uc005' .. '\uc01f'
        | '\uc021' .. '\uc03b'
        | '\uc03d' .. '\uc057'
        | '\uc059' .. '\uc073'
        | '\uc075' .. '\uc08f'
        | '\uc091' .. '\uc0ab'
        | '\uc0ad' .. '\uc0c7'
        | '\uc0c9' .. '\uc0e3'
        | '\uc0e5' .. '\uc0ff'
        | '\uc101' .. '\uc11b'
        | '\uc11d' .. '\uc137'
        | '\uc139' .. '\uc153'
        | '\uc155' .. '\uc16f'
        | '\uc171' .. '\uc18b'
        | '\uc18d' .. '\uc1a7'
        | '\uc1a9' .. '\uc1c3'
        | '\uc1c5' .. '\uc1df'
        | '\uc1e1' .. '\uc1fb'
        | '\uc1fd' .. '\uc217'
        | '\uc219' .. '\uc233'
        | '\uc235' .. '\uc24f'
        | '\uc251' .. '\uc26b'
        | '\uc26d' .. '\uc287'
        | '\uc289' .. '\uc2a3'
        | '\uc2a5' .. '\uc2bf'
        | '\uc2c1' .. '\uc2db'
        | '\uc2dd' .. '\uc2f7'
        | '\uc2f9' .. '\uc313'
        | '\uc315' .. '\uc32f'
        | '\uc331' .. '\uc34b'
        | '\uc34d' .. '\uc367'
        | '\uc369' .. '\uc383'
        | '\uc385' .. '\uc39f'
        | '\uc3a1' .. '\uc3bb'
        | '\uc3bd' .. '\uc3d7'
        | '\uc3d9' .. '\uc3f3'
        | '\uc3f5' .. '\uc40f'
        | '\uc411' .. '\uc42b'
        | '\uc42d' .. '\uc447'
        | '\uc449' .. '\uc463'
        | '\uc465' .. '\uc47f'
        | '\uc481' .. '\uc49b'
        | '\uc49d' .. '\uc4b7'
        | '\uc4b9' .. '\uc4d3'
        | '\uc4d5' .. '\uc4ef'
        | '\uc4f1' .. '\uc50b'
        | '\uc50d' .. '\uc527'
        | '\uc529' .. '\uc543'
        | '\uc545' .. '\uc55f'
        | '\uc561' .. '\uc57b'
        | '\uc57d' .. '\uc597'
        | '\uc599' .. '\uc5b3'
        | '\uc5b5' .. '\uc5cf'
        | '\uc5d1' .. '\uc5eb'
        | '\uc5ed' .. '\uc607'
        | '\uc609' .. '\uc623'
        | '\uc625' .. '\uc63f'
        | '\uc641' .. '\uc65b'
        | '\uc65d' .. '\uc677'
        | '\uc679' .. '\uc693'
        | '\uc695' .. '\uc6af'
        | '\uc6b1' .. '\uc6cb'
        | '\uc6cd' .. '\uc6e7'
        | '\uc6e9' .. '\uc703'
        | '\uc705' .. '\uc71f'
        | '\uc721' .. '\uc73b'
        | '\uc73d' .. '\uc757'
        | '\uc759' .. '\uc773'
        | '\uc775' .. '\uc78f'
        | '\uc791' .. '\uc7ab'
        | '\uc7ad' .. '\uc7c7'
        | '\uc7c9' .. '\uc7e3'
        | '\uc7e5' .. '\uc7ff'
        | '\uc801' .. '\uc81b'
        | '\uc81d' .. '\uc837'
        | '\uc839' .. '\uc853'
        | '\uc855' .. '\uc86f'
        | '\uc871' .. '\uc88b'
        | '\uc88d' .. '\uc8a7'
        | '\uc8a9' .. '\uc8c3'
        | '\uc8c5' .. '\uc8df'
        | '\uc8e1' .. '\uc8fb'
        | '\uc8fd' .. '\uc917'
        | '\uc919' .. '\uc933'
        | '\uc935' .. '\uc94f'
        | '\uc951' .. '\uc96b'
        | '\uc96d' .. '\uc987'
        | '\uc989' .. '\uc9a3'
        | '\uc9a5' .. '\uc9bf'
        | '\uc9c1' .. '\uc9db'
        | '\uc9dd' .. '\uc9f7'
        | '\uc9f9' .. '\uca13'
        | '\uca15' .. '\uca2f'
        | '\uca31' .. '\uca4b'
        | '\uca4d' .. '\uca67'
        | '\uca69' .. '\uca83'
        | '\uca85' .. '\uca9f'
        | '\ucaa1' .. '\ucabb'
        | '\ucabd' .. '\ucad7'
        | '\ucad9' .. '\ucaf3'
        | '\ucaf5' .. '\ucb0f'
        | '\ucb11' .. '\ucb2b'
        | '\ucb2d' .. '\ucb47'
        | '\ucb49' .. '\ucb63'
        | '\ucb65' .. '\ucb7f'
        | '\ucb81' .. '\ucb9b'
        | '\ucb9d' .. '\ucbb7'
        | '\ucbb9' .. '\ucbd3'
        | '\ucbd5' .. '\ucbef'
        | '\ucbf1' .. '\ucc0b'
        | '\ucc0d' .. '\ucc27'
        | '\ucc29' .. '\ucc43'
        | '\ucc45' .. '\ucc5f'
        | '\ucc61' .. '\ucc7b'
        | '\ucc7d' .. '\ucc97'
        | '\ucc99' .. '\uccb3'
        | '\uccb5' .. '\ucccf'
        | '\uccd1' .. '\ucceb'
        | '\ucced' .. '\ucd07'
        | '\ucd09' .. '\ucd23'
        | '\ucd25' .. '\ucd3f'
        | '\ucd41' .. '\ucd5b'
        | '\ucd5d' .. '\ucd77'
        | '\ucd79' .. '\ucd93'
        | '\ucd95' .. '\ucdaf'
        | '\ucdb1' .. '\ucdcb'
        | '\ucdcd' .. '\ucde7'
        | '\ucde9' .. '\uce03'
        | '\uce05' .. '\uce1f'
        | '\uce21' .. '\uce3b'
        | '\uce3d' .. '\uce57'
        | '\uce59' .. '\uce73'
        | '\uce75' .. '\uce8f'
        | '\uce91' .. '\uceab'
        | '\ucead' .. '\ucec7'
        | '\ucec9' .. '\ucee3'
        | '\ucee5' .. '\uceff'
        | '\ucf01' .. '\ucf1b'
        | '\ucf1d' .. '\ucf37'
        | '\ucf39' .. '\ucf53'
        | '\ucf55' .. '\ucf6f'
        | '\ucf71' .. '\ucf8b'
        | '\ucf8d' .. '\ucfa7'
        | '\ucfa9' .. '\ucfc3'
        | '\ucfc5' .. '\ucfdf'
        | '\ucfe1' .. '\ucffb'
        | '\ucffd' .. '\ud017'
        | '\ud019' .. '\ud033'
        | '\ud035' .. '\ud04f'
        | '\ud051' .. '\ud06b'
        | '\ud06d' .. '\ud087'
        | '\ud089' .. '\ud0a3'
        | '\ud0a5' .. '\ud0bf'
        | '\ud0c1' .. '\ud0db'
        | '\ud0dd' .. '\ud0f7'
        | '\ud0f9' .. '\ud113'
        | '\ud115' .. '\ud12f'
        | '\ud131' .. '\ud14b'
        | '\ud14d' .. '\ud167'
        | '\ud169' .. '\ud183'
        | '\ud185' .. '\ud19f'
        | '\ud1a1' .. '\ud1bb'
        | '\ud1bd' .. '\ud1d7'
        | '\ud1d9' .. '\ud1f3'
        | '\ud1f5' .. '\ud20f'
        | '\ud211' .. '\ud22b'
        | '\ud22d' .. '\ud247'
        | '\ud249' .. '\ud263'
        | '\ud265' .. '\ud27f'
        | '\ud281' .. '\ud29b'
        | '\ud29d' .. '\ud2b7'
        | '\ud2b9' .. '\ud2d3'
        | '\ud2d5' .. '\ud2ef'
        | '\ud2f1' .. '\ud30b'
        | '\ud30d' .. '\ud327'
        | '\ud329' .. '\ud343'
        | '\ud345' .. '\ud35f'
        | '\ud361' .. '\ud37b'
        | '\ud37d' .. '\ud397'
        | '\ud399' .. '\ud3b3'
        | '\ud3b5' .. '\ud3cf'
        | '\ud3d1' .. '\ud3eb'
        | '\ud3ed' .. '\ud407'
        | '\ud409' .. '\ud423'
        | '\ud425' .. '\ud43f'
        | '\ud441' .. '\ud45b'
        | '\ud45d' .. '\ud477'
        | '\ud479' .. '\ud493'
        | '\ud495' .. '\ud4af'
        | '\ud4b1' .. '\ud4cb'
        | '\ud4cd' .. '\ud4e7'
        | '\ud4e9' .. '\ud503'
        | '\ud505' .. '\ud51f'
        | '\ud521' .. '\ud53b'
        | '\ud53d' .. '\ud557'
        | '\ud559' .. '\ud573'
        | '\ud575' .. '\ud58f'
        | '\ud591' .. '\ud5ab'
        | '\ud5ad' .. '\ud5c7'
        | '\ud5c9' .. '\ud5e3'
        | '\ud5e5' .. '\ud5ff'
        | '\ud601' .. '\ud61b'
        | '\ud61d' .. '\ud637'
        | '\ud639' .. '\ud653'
        | '\ud655' .. '\ud66f'
        | '\ud671' .. '\ud68b'
        | '\ud68d' .. '\ud6a7'
        | '\ud6a9' .. '\ud6c3'
        | '\ud6c5' .. '\ud6df'
        | '\ud6e1' .. '\ud6fb'
        | '\ud6fd' .. '\ud717'
        | '\ud719' .. '\ud733'
        | '\ud735' .. '\ud74f'
        | '\ud751' .. '\ud76b'
        | '\ud76d' .. '\ud787'
        | '\ud789' .. '\ud7a3'
          => true,
        _ => false
    }
}

pub fn is_regional_indicator(c: char) -> bool {
    match c {
          '\U0001f1e6' .. '\U0001f1ff'
          => true,
        _ => false
    }
}

pub fn is_spacing_mark(c: char) -> bool {
    match c {
          '\u0903'
        | '\u093b'
        | '\u093e' .. '\u0940'
        | '\u0949' .. '\u094c'
        | '\u094e' .. '\u094f'
        | '\u0982' .. '\u0983'
        | '\u09bf' .. '\u09c0'
        | '\u09c7' .. '\u09c8'
        | '\u09cb' .. '\u09cc'
        | '\u0a03'
        | '\u0a3e' .. '\u0a40'
        | '\u0a83'
        | '\u0abe' .. '\u0ac0'
        | '\u0ac9'
        | '\u0acb' .. '\u0acc'
        | '\u0b02' .. '\u0b03'
        | '\u0b40'
        | '\u0b47' .. '\u0b48'
        | '\u0b4b' .. '\u0b4c'
        | '\u0bbf'
        | '\u0bc1' .. '\u0bc2'
        | '\u0bc6' .. '\u0bc8'
        | '\u0bca' .. '\u0bcc'
        | '\u0c01' .. '\u0c03'
        | '\u0c41' .. '\u0c44'
        | '\u0c82' .. '\u0c83'
        | '\u0cbe'
        | '\u0cc0' .. '\u0cc1'
        | '\u0cc3' .. '\u0cc4'
        | '\u0cc7' .. '\u0cc8'
        | '\u0cca' .. '\u0ccb'
        | '\u0d02' .. '\u0d03'
        | '\u0d3f' .. '\u0d40'
        | '\u0d46' .. '\u0d48'
        | '\u0d4a' .. '\u0d4c'
        | '\u0d82' .. '\u0d83'
        | '\u0dd0' .. '\u0dd1'
        | '\u0dd8' .. '\u0dde'
        | '\u0df2' .. '\u0df3'
        | '\u0e33'
        | '\u0eb3'
        | '\u0f3e' .. '\u0f3f'
        | '\u0f7f'
        | '\u1031'
        | '\u103b' .. '\u103c'
        | '\u1056' .. '\u1057'
        | '\u1084'
        | '\u17b6'
        | '\u17be' .. '\u17c5'
        | '\u17c7' .. '\u17c8'
        | '\u1923' .. '\u1926'
        | '\u1929' .. '\u192b'
        | '\u1930' .. '\u1931'
        | '\u1933' .. '\u1938'
        | '\u19b5' .. '\u19b7'
        | '\u19ba'
        | '\u1a19' .. '\u1a1b'
        | '\u1a55'
        | '\u1a57'
        | '\u1a6d' .. '\u1a72'
        | '\u1b04'
        | '\u1b35'
        | '\u1b3b'
        | '\u1b3d' .. '\u1b41'
        | '\u1b43' .. '\u1b44'
        | '\u1b82'
        | '\u1ba1'
        | '\u1ba6' .. '\u1ba7'
        | '\u1baa'
        | '\u1bac' .. '\u1bad'
        | '\u1be7'
        | '\u1bea' .. '\u1bec'
        | '\u1bee'
        | '\u1bf2' .. '\u1bf3'
        | '\u1c24' .. '\u1c2b'
        | '\u1c34' .. '\u1c35'
        | '\u1ce1'
        | '\u1cf2' .. '\u1cf3'
        | '\ua823' .. '\ua824'
        | '\ua827'
        | '\ua880' .. '\ua881'
        | '\ua8b4' .. '\ua8c3'
        | '\ua952' .. '\ua953'
        | '\ua983'
        | '\ua9b4' .. '\ua9b5'
        | '\ua9ba' .. '\ua9bb'
        | '\ua9bd' .. '\ua9c0'
        | '\uaa2f' .. '\uaa30'
        | '\uaa33' .. '\uaa34'
        | '\uaa4d'
        | '\uaaeb'
        | '\uaaee' .. '\uaaef'
        | '\uaaf5'
        | '\uabe3' .. '\uabe4'
        | '\uabe6' .. '\uabe7'
        | '\uabe9' .. '\uabea'
        | '\uabec'
        | '\U00011000'
        | '\U00011002'
        | '\U00011082'
        | '\U000110b0' .. '\U000110b2'
        | '\U000110b7' .. '\U000110b8'
        | '\U0001112c'
        | '\U00011182'
        | '\U000111b3' .. '\U000111b5'
        | '\U000111bf' .. '\U000111c0'
        | '\U000116ac'
        | '\U000116ae' .. '\U000116af'
        | '\U000116b6'
        | '\U00016f51' .. '\U00016f7e'
        | '\U0001d166'
        | '\U0001d16d'
          => true,
        _ => false
    }
}

pub fn is_t(c: char) -> bool {
    match c {
          '\u11a8' .. '\u11ff'
        | '\ud7cb' .. '\ud7fb'
          => true,
        _ => false
    }
}

pub fn is_v(c: char) -> bool {
    match c {
          '\u1160' .. '\u11a7'
        | '\ud7b0' .. '\ud7c6'
          => true,
        _ => false
    }
}
