@0x9512134be87cbc78;

struct HTTP {
	protocol @0 :Protocol;
	enum Protocol {
		unknown @0;
		http10 @1;
		http11 @2;
		max @3;
	}

	status @1 :UInt16;

	hostStatus @2 :UInt16;

	upStatus @3 :UInt16;

	method @4 :Method;
	enum Method {
		unknown @0;
		get @1;
		post @2;
		delete @3;
		put @4;
		head @5;
		purge @6;
		options @7;
		propfind @8;
		mkcol @9;
		patch @10;
		max @11;
	}

	contentType @5 :Text;
	userAgent @6 :Text;
	referer @7 :Text;
	requestURI @8 :Text;
}

enum CacheStatus {
	unknown @0;
	miss @1;
	expired @2;
	hit @3;
	max @4;
}

struct Origin {
	ip @0 :Text;
	port @1 :UInt16;
	hostname @2 :Text;
	protocol @3 :Protocol;
	enum Protocol {
		unknown @0;
		http @1;
		https @2;
		max @3;
	}
}

enum ZonePlan {
	unknown @0;
	free @1;
	pro @2;
	biz @3;
	ent @4;
	max @5;
}

enum Country {
	unknown @0;
	# Anonymous Proxy
	a1 @1;
	# Satellite Provider
	a2 @2;
	# Other Country
	o1 @3;
	# Andorra
	ad @4;
	# United Arab Emirates
	ae @5;
	# Afghanistan
	af @6;
	# Antigua and Barbuda
	ag @7;
	# Anguilla
	ai @8;
	# Albania
	al @9;
	# Armenia
	am @10;
	# Angola
	ao @11;
	# Asia/Pacific Region
	ap @12;
	# Antarctica
	aq @13;
	# Argentina
	ar @14;
	# American Samoa
	as @15;
	# Austria
	at @16;
	# Australia
	au @17;
	# Aruba
	aw @18;
	# Aland Islands
	ax @19;
	# Azerbaijan
	az @20;
	# Bosnia and Herzegovina
	ba @21;
	# Barbados
	bb @22;
	# Bangladesh
	bd @23;
	# Belgium
	be @24;
	# Burkina Faso
	bf @25;
	# Bulgaria
	bg @26;
	# Bahrain
	bh @27;
	# Burundi
	bi @28;
	# Benin
	bj @29;
	# Saint Bartelemey
	bl @30;
	# Bermuda
	bm @31;
	# Brunei Darussalam
	bn @32;
	# Bolivia
	bo @33;
	# Bonaire, Saint Eustatius and Saba
	bq @34;
	# Brazil
	br @35;
	# Bahamas
	bs @36;
	# Bhutan
	bt @37;
	# Bouvet Island
	bv @38;
	# Botswana
	bw @39;
	# Belarus
	by @40;
	# Belize
	bz @41;
	# Canada
	ca @42;
	# Cocos (Keeling) Islands
	cc @43;
	# Congo, The Democratic Republic of the
	cd @44;
	# Central African Republic
	cf @45;
	# Congo
	cg @46;
	# Switzerland
	ch @47;
	# Cote d'Ivoire
	ci @48;
	# Cook Islands
	ck @49;
	# Chile
	cl @50;
	# Cameroon
	cm @51;
	# China
	cn @52;
	# Colombia
	co @53;
	# Costa Rica
	cr @54;
	# Cuba
	cu @55;
	# Cape Verde
	cv @56;
	# Curacao
	cw @57;
	# Christmas Island
	cx @58;
	# Cyprus
	cy @59;
	# Czech Republic
	cz @60;
	# Germany
	de @61;
	# Djibouti
	dj @62;
	# Denmark
	dk @63;
	# Dominica
	dm @64;
	# Dominican Republic
	do @65;
	# Algeria
	dz @66;
	# Ecuador
	ec @67;
	# Estonia
	ee @68;
	# Egypt
	eg @69;
	# Western Sahara
	eh @70;
	# Eritrea
	er @71;
	# Spain
	es @72;
	# Ethiopia
	et @73;
	# Europe
	eu @74;
	# Finland
	fi @75;
	# Fiji
	fj @76;
	# Falkland Islands (Malvinas)
	fk @77;
	# Micronesia, Federated States of
	fm @78;
	# Faroe Islands
	fo @79;
	# France
	fr @80;
	# Gabon
	ga @81;
	# United Kingdom
	gb @82;
	# Grenada
	gd @83;
	# Georgia
	ge @84;
	# French Guiana
	gf @85;
	# Guernsey
	gg @86;
	# Ghana
	gh @87;
	# Gibraltar
	gi @88;
	# Greenland
	gl @89;
	# Gambia
	gm @90;
	# Guinea
	gn @91;
	# Guadeloupe
	gp @92;
	# Equatorial Guinea
	gq @93;
	# Greece
	gr @94;
	# South Georgia and the South Sandwich Islands
	gs @95;
	# Guatemala
	gt @96;
	# Guam
	gu @97;
	# Guinea-Bissau
	gw @98;
	# Guyana
	gy @99;
	# Hong Kong
	hk @100;
	# Heard Island and McDonald Islands
	hm @101;
	# Honduras
	hn @102;
	# Croatia
	hr @103;
	# Haiti
	ht @104;
	# Hungary
	hu @105;
	# Indonesia
	id @106;
	# Ireland
	ie @107;
	# Israel
	il @108;
	# Isle of Man
	im @109;
	# India
	in @110;
	# British Indian Ocean Territory
	io @111;
	# Iraq
	iq @112;
	# Iran, Islamic Republic of
	ir @113;
	# Iceland
	is @114;
	# Italy
	it @115;
	# Jersey
	je @116;
	# Jamaica
	jm @117;
	# Jordan
	jo @118;
	# Japan
	jp @119;
	# Kenya
	ke @120;
	# Kyrgyzstan
	kg @121;
	# Cambodia
	kh @122;
	# Kiribati
	ki @123;
	# Comoros
	km @124;
	# Saint Kitts and Nevis
	kn @125;
	# Korea, Democratic People's Republic of
	kp @126;
	# Korea, Republic of
	kr @127;
	# Kuwait
	kw @128;
	# Cayman Islands
	ky @129;
	# Kazakhstan
	kz @130;
	# Lao People's Democratic Republic
	la @131;
	# Lebanon
	lb @132;
	# Saint Lucia
	lc @133;
	# Liechtenstein
	li @134;
	# Sri Lanka
	lk @135;
	# Liberia
	lr @136;
	# Lesotho
	ls @137;
	# Lithuania
	lt @138;
	# Luxembourg
	lu @139;
	# Latvia
	lv @140;
	# Libyan Arab Jamahiriya
	ly @141;
	# Morocco
	ma @142;
	# Monaco
	mc @143;
	# Moldova, Republic of
	md @144;
	# Montenegro
	me @145;
	# Saint Martin
	mf @146;
	# Madagascar
	mg @147;
	# Marshall Islands
	mh @148;
	# Macedonia
	mk @149;
	# Mali
	ml @150;
	# Myanmar
	mm @151;
	# Mongolia
	mn @152;
	# Macao
	mo @153;
	# Northern Mariana Islands
	mp @154;
	# Martinique
	mq @155;
	# Mauritania
	mr @156;
	# Montserrat
	ms @157;
	# Malta
	mt @158;
	# Mauritius
	mu @159;
	# Maldives
	mv @160;
	# Malawi
	mw @161;
	# Mexico
	mx @162;
	# Malaysia
	my @163;
	# Mozambique
	mz @164;
	# Namibia
	na @165;
	# New Caledonia
	nc @166;
	# Niger
	ne @167;
	# Norfolk Island
	nf @168;
	# Nigeria
	ng @169;
	# Nicaragua
	ni @170;
	# Netherlands
	nl @171;
	# Norway
	no @172;
	# Nepal
	np @173;
	# Nauru
	nr @174;
	# Niue
	nu @175;
	# New Zealand
	nz @176;
	# Oman
	om @177;
	# Panama
	pa @178;
	# Peru
	pe @179;
	# French Polynesia
	pf @180;
	# Papua New Guinea
	pg @181;
	# Philippines
	ph @182;
	# Pakistan
	pk @183;
	# Poland
	pl @184;
	# Saint Pierre and Miquelon
	pm @185;
	# Pitcairn
	pn @186;
	# Puerto Rico
	pr @187;
	# Palestinian Territory
	ps @188;
	# Portugal
	pt @189;
	# Palau
	pw @190;
	# Paraguay
	py @191;
	# Qatar
	qa @192;
	# Reunion
	re @193;
	# Romania
	ro @194;
	# Serbia
	rs @195;
	# Russian Federation
	ru @196;
	# Rwanda
	rw @197;
	# Saudi Arabia
	sa @198;
	# Solomon Islands
	sb @199;
	# Seychelles
	sc @200;
	# Sudan
	sd @201;
	# Sweden
	se @202;
	# Singapore
	sg @203;
	# Saint Helena
	sh @204;
	# Slovenia
	si @205;
	# Svalbard and Jan Mayen
	sj @206;
	# Slovakia
	sk @207;
	# Sierra Leone
	sl @208;
	# San Marino
	sm @209;
	# Senegal
	sn @210;
	# Somalia
	so @211;
	# Suriname
	sr @212;
	# South Sudan
	ss @213;
	# Sao Tome and Principe
	st @214;
	# El Salvador
	sv @215;
	# Sint Maarten
	sx @216;
	# Syrian Arab Republic
	sy @217;
	# Swaziland
	sz @218;
	# Turks and Caicos Islands
	tc @219;
	# Chad
	td @220;
	# French Southern Territories
	tf @221;
	# Togo
	tg @222;
	# Thailand
	th @223;
	# Tajikistan
	tj @224;
	# Tokelau
	tk @225;
	# Timor-Leste
	tl @226;
	# Turkmenistan
	tm @227;
	# Tunisia
	tn @228;
	# Tonga
	to @229;
	# Turkey
	tr @230;
	# Trinidad and Tobago
	tt @231;
	# Tuvalu
	tv @232;
	# Taiwan
	tw @233;
	# Tanzania, United Republic of
	tz @234;
	# Ukraine
	ua @235;
	# Uganda
	ug @236;
	# United States Minor Outlying Islands
	um @237;
	# United States
	us @238;
	# Uruguay
	uy @239;
	# Uzbekistan
	uz @240;
	# Holy See (Vatican City State)
	va @241;
	# Saint Vincent and the Grenadines
	vc @242;
	# Venezuela
	ve @243;
	# Virgin Islands, British
	vg @244;
	# Virgin Islands, U.S.
	vi @245;
	# Vietnam
	vn @246;
	# Vanuatu
	vu @247;
	# Wallis and Futuna
	wf @248;
	# Samoa
	ws @249;
	# Other Countries
	xx @250;
	# Yemen
	ye @251;
	# Mayotte
	yt @252;
	# South Africa
	za @253;
	# Zambia
	zm @254;
	# Zimbabwe
	zw @255;
	max @256;
}

struct Log {
	timestamp @0 :Int64;
	zoneId @1 :UInt32;
	zonePlan @2 :ZonePlan;
	http @3 :HTTP;
	origin @4 :Origin;
	country @5 :Country;
	cacheStatus @6 :CacheStatus;
	serverIp @7 :Text;
	serverName @8 :Text;
	remoteIp @9 :Text;
	bytesDlv @10 :UInt64;
	rayId @11 :Text;
}
