#!/usr/bin/env python3

import re
import sys

# https://www.itu.int/en/ITU-R/terrestrial/fmd/Pages/mid.aspx
MID = """
201	Albania (Republic of)
202	Andorra (Principality of)
203	Austria
204	Portugal - Azores
205	Belgium
206	Belarus (Republic of)
207	Bulgaria (Republic of)
208	Vatican City State
209	Cyprus (Republic of)
210	Cyprus (Republic of)
211	Germany (Federal Republic of)
212	Cyprus (Republic of)
213	Georgia
214	Moldova (Republic of)
215	Malta
216	Armenia (Republic of)
218	Germany (Federal Republic of)
219	Denmark
220	Denmark
224	Spain
225	Spain
226	France
227	France
228	France
229	Malta
230	Finland
231	Denmark - Faroe Islands
232	United Kingdom of Great Britain and Northern Ireland
233	United Kingdom of Great Britain and Northern Ireland
234	United Kingdom of Great Britain and Northern Ireland
235	United Kingdom of Great Britain and Northern Ireland
236	United Kingdom of Great Britain and Northern Ireland - Gibraltar
237	Greece
238	Croatia (Republic of)
239	Greece
240	Greece
241	Greece
242	Morocco (Kingdom of)
243	Hungary
244	Netherlands (Kingdom of the)
245	Netherlands (Kingdom of the)
246	Netherlands (Kingdom of the)
247	Italy
248	Malta
249	Malta
250	Ireland
251	Iceland
252	Liechtenstein (Principality of)
253	Luxembourg
254	Monaco (Principality of)
255	Portugal - Madeira
256	Malta
257	Norway
258	Norway
259	Norway
261	Poland (Republic of)
262	Montenegro
263	Portugal
264	Romania
265	Sweden
266	Sweden
267	Slovak Republic
268	San Marino (Republic of)
269	Switzerland (Confederation of)
270	Czech Republic
271	Turkey
272	Ukraine
273	Russian Federation
274	North Macedonia (Republic of)
275	Latvia (Republic of)
276	Estonia (Republic of)
277	Lithuania (Republic of)
278	Slovenia (Republic of)
279	Serbia (Republic of)
301	United Kingdom of Great Britain and Northern Ireland - Anguilla
303	United States of America - Alaska (State of)
304	Antigua and Barbuda
305	Antigua and Barbuda
306	Netherlands (Kingdom of the) - Bonaire, Sint Eustatius and Saba
306	Netherlands (Kingdom of the) - Curaçao
306	Netherlands (Kingdom of the) - Sint Maarten (Dutch part)
307	Netherlands (Kingdom of the) - Aruba
308	Bahamas (Commonwealth of the)
309	Bahamas (Commonwealth of the)
310	United Kingdom of Great Britain and Northern Ireland - Bermuda
311	Bahamas (Commonwealth of the)
312	Belize
314	Barbados
316	Canada
319	United Kingdom of Great Britain and Northern Ireland - Cayman Islands
321	Costa Rica
323	Cuba
325	Dominica (Commonwealth of)
327	Dominican Republic
329	France - Guadeloupe (French Department of)
330	Grenada
331	Denmark - Greenland
332	Guatemala (Republic of)
334	Honduras (Republic of)
336	Haiti (Republic of)
338	United States of America
339	Jamaica
341	Saint Kitts and Nevis (Federation of)
343	Saint Lucia
345	Mexico
347	France - Martinique (French Department of)
348	United Kingdom of Great Britain and Northern Ireland - Montserrat
350	Nicaragua
351	Panama (Republic of)
352	Panama (Republic of)
353	Panama (Republic of)
354	Panama (Republic of)
355	Panama (Republic of)
356	Panama (Republic of)
357	Panama (Republic of)
358	United States of America - Puerto Rico
359	El Salvador (Republic of)
361	France - Saint Pierre and Miquelon (Territorial Collectivity of)
362	Trinidad and Tobago
364	United Kingdom of Great Britain and Northern Ireland - Turks and Caicos Islands
366	United States of America
367	United States of America
368	United States of America
369	United States of America
370	Panama (Republic of)
371	Panama (Republic of)
372	Panama (Republic of)
373	Panama (Republic of)
374	Panama (Republic of)
375	Saint Vincent and the Grenadines
376	Saint Vincent and the Grenadines
377	Saint Vincent and the Grenadines
378	United Kingdom of Great Britain and Northern Ireland - British Virgin Islands
379	United States of America - United States Virgin Islands
401	Afghanistan
403	Saudi Arabia (Kingdom of)
405	Bangladesh (People's Republic of)
408	Bahrain (Kingdom of)
410	Bhutan (Kingdom of)
412	China (People's Republic of)
413	China (People's Republic of)
414	China (People's Republic of)
416	China (People's Republic of) - Taiwan (Province of China)
417	Sri Lanka (Democratic Socialist Republic of)
419	India (Republic of)
422	Iran (Islamic Republic of)
423	Azerbaijan (Republic of)
425	Iraq (Republic of)
428	Israel (State of)
431	Japan
432	Japan
434	Turkmenistan
436	Kazakhstan (Republic of)
437	Uzbekistan (Republic of)
438	Jordan (Hashemite Kingdom of)
440	Korea (Republic of)
441	Korea (Republic of)
443	State of Palestine (In accordance with Resolution 99 Rev. Dubai, 2018)
445	Democratic People's Republic of Korea
447	Kuwait (State of)
450	Lebanon
451	Kyrgyz Republic
453	China (People's Republic of) - Macao (Special Administrative Region of China)
455	Maldives (Republic of)
457	Mongolia
459	Nepal (Federal Democratic Republic of)
461	Oman (Sultanate of)
463	Pakistan (Islamic Republic of)
466	Qatar (State of)
468	Syrian Arab Republic
470	United Arab Emirates
471	United Arab Emirates
472	Tajikistan (Republic of)
473	Yemen (Republic of)
475	Yemen (Republic of)
477	China (People's Republic of) - Hong Kong (Special Administrative Region of China)
478	Bosnia and Herzegovina
501	France - Adelie Land
503	Australia
506	Myanmar (Union of)
508	Brunei Darussalam
510	Micronesia (Federated States of)
511	Palau (Republic of)
512	New Zealand
514	Cambodia (Kingdom of)
515	Cambodia (Kingdom of)
516	Australia - Christmas Island (Indian Ocean)
518	New Zealand - Cook Islands
520	Fiji (Republic of)
523	Australia - Cocos (Keeling) Islands
525	Indonesia (Republic of)
529	Kiribati (Republic of)
531	Lao People's Democratic Republic
533	Malaysia
536	United States of America - Northern Mariana Islands (Commonwealth of the)
538	Marshall Islands (Republic of the)
540	France - New Caledonia
542	New Zealand - Niue
544	Nauru (Republic of)
546	France - French Polynesia
548	Philippines (Republic of the)
550	Timor-Leste (Democratic Republic of)
553	Papua New Guinea
555	United Kingdom of Great Britain and Northern Ireland - Pitcairn Island
557	Solomon Islands
559	United States of America - American Samoa
561	Samoa (Independent State of)
563	Singapore (Republic of)
564	Singapore (Republic of)
565	Singapore (Republic of)
566	Singapore (Republic of)
567	Thailand
570	Tonga (Kingdom of)
572	Tuvalu
574	Viet Nam (Socialist Republic of)
576	Vanuatu (Republic of)
577	Vanuatu (Republic of)
578	France - Wallis and Futuna Islands
601	South Africa (Republic of)
603	Angola (Republic of)
605	Algeria (People's Democratic Republic of)
607	France - Saint Paul and Amsterdam Islands
608	United Kingdom of Great Britain and Northern Ireland - Ascension Island
609	Burundi (Republic of)
610	Benin (Republic of)
611	Botswana (Republic of)
612	Central African Republic
613	Cameroon (Republic of)
615	Congo (Republic of the)
616	Comoros (Union of the)
617	Cabo Verde (Republic of)
618	France - Crozet Archipelago
619	Côte d'Ivoire (Republic of)
620	Comoros (Union of the)
621	Djibouti (Republic of)
622	Egypt (Arab Republic of)
624	Ethiopia (Federal Democratic Republic of)
625	Eritrea
626	Gabonese Republic
627	Ghana
629	Gambia (Republic of the)
630	Guinea-Bissau (Republic of)
631	Equatorial Guinea (Republic of)
632	Guinea (Republic of)
633	Burkina Faso
634	Kenya (Republic of)
635	France - Kerguelen Islands
636	Liberia (Republic of)
637	Liberia (Republic of)
638	South Sudan (Republic of)
642	Libya (State of)
644	Lesotho (Kingdom of)
645	Mauritius (Republic of)
647	Madagascar (Republic of)
649	Mali (Republic of)
650	Mozambique (Republic of)
654	Mauritania (Islamic Republic of)
655	Malawi
656	Niger (Republic of the)
657	Nigeria (Federal Republic of)
659	Namibia (Republic of)
660	France - Reunion (French Department of)
661	Rwanda (Republic of)
662	Sudan (Republic of the)
663	Senegal (Republic of)
664	Seychelles (Republic of)
665	United Kingdom of Great Britain and Northern Ireland - Saint Helena
666	Somalia (Federal Republic of)
667	Sierra Leone
668	Sao Tome and Principe (Democratic Republic of)
669	Eswatini (Kingdom of)
670	Chad (Republic of)
671	Togolese Republic
672	Tunisia
674	Tanzania (United Republic of)
675	Uganda (Republic of)
676	Democratic Republic of the Congo
677	Tanzania (United Republic of)
678	Zambia (Republic of)
679	Zimbabwe (Republic of)
701	Argentine Republic
710	Brazil (Federative Republic of)
720	Bolivia (Plurinational State of)
725	Chile
730	Colombia (Republic of)
735	Ecuador
740	United Kingdom of Great Britain and Northern Ireland - Falkland Islands (Malvinas)
745	France - Guiana (French Department of)
750	Guyana
755	Paraguay (Republic of)
760	Peru
765	Suriname (Republic of)
770	Uruguay (Eastern Republic of)
775	Venezuela (Bolivarian Republic of)
"""

# https://www.iban.com/country-codes
ISO_3166 = """
Afghanistan 	AF 	AFG 	004
Albania 	AL 	ALB 	008
Algeria 	DZ 	DZA 	012
American Samoa 	AS 	ASM 	016
Andorra 	AD 	AND 	020
Angola 	AO 	AGO 	024
Anguilla 	AI 	AIA 	660
Antarctica 	AQ 	ATA 	010
Antigua and Barbuda 	AG 	ATG 	028
Argentina 	AR 	ARG 	032
Armenia 	AM 	ARM 	051
Aruba 	AW 	ABW 	533
Australia 	AU 	AUS 	036
Austria 	AT 	AUT 	040
Azerbaijan 	AZ 	AZE 	031
Bahamas (the) 	BS 	BHS 	044
Bahrain 	BH 	BHR 	048
Bangladesh 	BD 	BGD 	050
Barbados 	BB 	BRB 	052
Belarus 	BY 	BLR 	112
Belgium 	BE 	BEL 	056
Belize 	BZ 	BLZ 	084
Benin 	BJ 	BEN 	204
Bermuda 	BM 	BMU 	060
Bhutan 	BT 	BTN 	064
Bolivia (Plurinational State of) 	BO 	BOL 	068
Bonaire, Sint Eustatius and Saba 	BQ 	BES 	535
Bosnia and Herzegovina 	BA 	BIH 	070
Botswana 	BW 	BWA 	072
Bouvet Island 	BV 	BVT 	074
Brazil 	BR 	BRA 	076
British Indian Ocean Territory (the) 	IO 	IOT 	086
Brunei Darussalam 	BN 	BRN 	096
Bulgaria 	BG 	BGR 	100
Burkina Faso 	BF 	BFA 	854
Burundi 	BI 	BDI 	108
Cabo Verde 	CV 	CPV 	132
Cambodia 	KH 	KHM 	116
Cameroon 	CM 	CMR 	120
Canada 	CA 	CAN 	124
Cayman Islands (the) 	KY 	CYM 	136
Central African Republic (the) 	CF 	CAF 	140
Chad 	TD 	TCD 	148
Chile 	CL 	CHL 	152
China 	CN 	CHN 	156
Christmas Island 	CX 	CXR 	162
Cocos (Keeling) Islands (the) 	CC 	CCK 	166
Colombia 	CO 	COL 	170
Comoros (the) 	KM 	COM 	174
Congo (the Democratic Republic of the) 	CD 	COD 	180
Congo (the) 	CG 	COG 	178
Cook Islands (the) 	CK 	COK 	184
Costa Rica 	CR 	CRI 	188
Croatia 	HR 	HRV 	191
Cuba 	CU 	CUB 	192
Curaçao 	CW 	CUW 	531
Cyprus 	CY 	CYP 	196
Czechia 	CZ 	CZE 	203
Côte d'Ivoire 	CI 	CIV 	384
Denmark 	DK 	DNK 	208
Djibouti 	DJ 	DJI 	262
Dominica 	DM 	DMA 	212
Dominican Republic (the) 	DO 	DOM 	214
Ecuador 	EC 	ECU 	218
Egypt 	EG 	EGY 	818
El Salvador 	SV 	SLV 	222
Equatorial Guinea 	GQ 	GNQ 	226
Eritrea 	ER 	ERI 	232
Estonia 	EE 	EST 	233
Eswatini 	SZ 	SWZ 	748
Ethiopia 	ET 	ETH 	231
Falkland Islands (the) [Malvinas] 	FK 	FLK 	238
Faroe Islands (the) 	FO 	FRO 	234
Fiji 	FJ 	FJI 	242
Finland 	FI 	FIN 	246
France 	FR 	FRA 	250
French Guiana 	GF 	GUF 	254
French Polynesia 	PF 	PYF 	258
French Southern Territories (the) 	TF 	ATF 	260
Gabon 	GA 	GAB 	266
Gambia (the) 	GM 	GMB 	270
Georgia 	GE 	GEO 	268
Germany 	DE 	DEU 	276
Ghana 	GH 	GHA 	288
Gibraltar 	GI 	GIB 	292
Greece 	GR 	GRC 	300
Greenland 	GL 	GRL 	304
Grenada 	GD 	GRD 	308
Guadeloupe 	GP 	GLP 	312
Guam 	GU 	GUM 	316
Guatemala 	GT 	GTM 	320
Guernsey 	GG 	GGY 	831
Guinea 	GN 	GIN 	324
Guinea-Bissau 	GW 	GNB 	624
Guyana 	GY 	GUY 	328
Haiti 	HT 	HTI 	332
Heard Island and McDonald Islands 	HM 	HMD 	334
Holy See (the) 	VA 	VAT 	336
Honduras 	HN 	HND 	340
Hong Kong 	HK 	HKG 	344
Hungary 	HU 	HUN 	348
Iceland 	IS 	ISL 	352
India 	IN 	IND 	356
Indonesia 	ID 	IDN 	360
Iran (Islamic Republic of) 	IR 	IRN 	364
Iraq 	IQ 	IRQ 	368
Ireland 	IE 	IRL 	372
Isle of Man 	IM 	IMN 	833
Israel 	IL 	ISR 	376
Italy 	IT 	ITA 	380
Jamaica 	JM 	JAM 	388
Japan 	JP 	JPN 	392
Jersey 	JE 	JEY 	832
Jordan 	JO 	JOR 	400
Kazakhstan 	KZ 	KAZ 	398
Kenya 	KE 	KEN 	404
Kiribati 	KI 	KIR 	296
Korea (the Democratic People's Republic of) 	KP 	PRK 	408
Korea (the Republic of) 	KR 	KOR 	410
Kuwait 	KW 	KWT 	414
Kyrgyzstan 	KG 	KGZ 	417
Lao People's Democratic Republic (the) 	LA 	LAO 	418
Latvia 	LV 	LVA 	428
Lebanon 	LB 	LBN 	422
Lesotho 	LS 	LSO 	426
Liberia 	LR 	LBR 	430
Libya 	LY 	LBY 	434
Liechtenstein 	LI 	LIE 	438
Lithuania 	LT 	LTU 	440
Luxembourg 	LU 	LUX 	442
Macao 	MO 	MAC 	446
Madagascar 	MG 	MDG 	450
Malawi 	MW 	MWI 	454
Malaysia 	MY 	MYS 	458
Maldives 	MV 	MDV 	462
Mali 	ML 	MLI 	466
Malta 	MT 	MLT 	470
Marshall Islands (the) 	MH 	MHL 	584
Martinique 	MQ 	MTQ 	474
Mauritania 	MR 	MRT 	478
Mauritius 	MU 	MUS 	480
Mayotte 	YT 	MYT 	175
Mexico 	MX 	MEX 	484
Micronesia (Federated States of) 	FM 	FSM 	583
Moldova (the Republic of) 	MD 	MDA 	498
Monaco 	MC 	MCO 	492
Mongolia 	MN 	MNG 	496
Montenegro 	ME 	MNE 	499
Montserrat 	MS 	MSR 	500
Morocco 	MA 	MAR 	504
Mozambique 	MZ 	MOZ 	508
Myanmar 	MM 	MMR 	104
Namibia 	NA 	NAM 	516
Nauru 	NR 	NRU 	520
Nepal 	NP 	NPL 	524
Netherlands (the) 	NL 	NLD 	528
New Caledonia 	NC 	NCL 	540
New Zealand 	NZ 	NZL 	554
Nicaragua 	NI 	NIC 	558
Niger (the) 	NE 	NER 	562
Nigeria 	NG 	NGA 	566
Niue 	NU 	NIU 	570
Norfolk Island 	NF 	NFK 	574
Northern Mariana Islands (the) 	MP 	MNP 	580
Norway 	NO 	NOR 	578
Oman 	OM 	OMN 	512
Pakistan 	PK 	PAK 	586
Palau 	PW 	PLW 	585
Palestine, State of 	PS 	PSE 	275
Panama 	PA 	PAN 	591
Papua New Guinea 	PG 	PNG 	598
Paraguay 	PY 	PRY 	600
Peru 	PE 	PER 	604
Philippines (the) 	PH 	PHL 	608
Pitcairn 	PN 	PCN 	612
Poland 	PL 	POL 	616
Portugal 	PT 	PRT 	620
Puerto Rico 	PR 	PRI 	630
Qatar 	QA 	QAT 	634
Republic of North Macedonia 	MK 	MKD 	807
Romania 	RO 	ROU 	642
Russian Federation (the) 	RU 	RUS 	643
Rwanda 	RW 	RWA 	646
Réunion 	RE 	REU 	638
Saint Barthélemy 	BL 	BLM 	652
Saint Helena, Ascension and Tristan da Cunha 	SH 	SHN 	654
Saint Kitts and Nevis 	KN 	KNA 	659
Saint Lucia 	LC 	LCA 	662
Saint Martin (French part) 	MF 	MAF 	663
Saint Pierre and Miquelon 	PM 	SPM 	666
Saint Vincent and the Grenadines 	VC 	VCT 	670
Samoa 	WS 	WSM 	882
San Marino 	SM 	SMR 	674
Sao Tome and Principe 	ST 	STP 	678
Saudi Arabia 	SA 	SAU 	682
Senegal 	SN 	SEN 	686
Serbia 	RS 	SRB 	688
Seychelles 	SC 	SYC 	690
Sierra Leone 	SL 	SLE 	694
Singapore 	SG 	SGP 	702
Sint Maarten (Dutch part) 	SX 	SXM 	534
Slovakia 	SK 	SVK 	703
Slovenia 	SI 	SVN 	705
Solomon Islands 	SB 	SLB 	090
Somalia 	SO 	SOM 	706
South Africa 	ZA 	ZAF 	710
South Georgia and the South Sandwich Islands 	GS 	SGS 	239
South Sudan 	SS 	SSD 	728
Spain 	ES 	ESP 	724
Sri Lanka 	LK 	LKA 	144
Sudan (the) 	SD 	SDN 	729
Suriname 	SR 	SUR 	740
Svalbard and Jan Mayen 	SJ 	SJM 	744
Sweden 	SE 	SWE 	752
Switzerland 	CH 	CHE 	756
Syrian Arab Republic 	SY 	SYR 	760
Taiwan (Province of China) 	TW 	TWN 	158
Tajikistan 	TJ 	TJK 	762
Tanzania, United Republic of 	TZ 	TZA 	834
Thailand 	TH 	THA 	764
Timor-Leste 	TL 	TLS 	626
Togo 	TG 	TGO 	768
Tokelau 	TK 	TKL 	772
Tonga 	TO 	TON 	776
Trinidad and Tobago 	TT 	TTO 	780
Tunisia 	TN 	TUN 	788
Turkey 	TR 	TUR 	792
Turkmenistan 	TM 	TKM 	795
Turks and Caicos Islands (the) 	TC 	TCA 	796
Tuvalu 	TV 	TUV 	798
Uganda 	UG 	UGA 	800
Ukraine 	UA 	UKR 	804
United Arab Emirates (the) 	AE 	ARE 	784
United Kingdom of Great Britain and Northern Ireland (the) 	GB 	GBR 	826
United States Minor Outlying Islands (the) 	UM 	UMI 	581
United States of America (the) 	US 	USA 	840
Uruguay 	UY 	URY 	858
Uzbekistan 	UZ 	UZB 	860
Vanuatu 	VU 	VUT 	548
Venezuela (Bolivarian Republic of) 	VE 	VEN 	862
Viet Nam 	VN 	VNM 	704
Virgin Islands (British) 	VG 	VGB 	092
Virgin Islands (U.S.) 	VI 	VIR 	850
Wallis and Futuna 	WF 	WLF 	876
Western Sahara 	EH 	ESH 	732
Yemen 	YE 	YEM 	887
Zambia 	ZM 	ZMB 	894
Zimbabwe 	ZW 	ZWE 	716
Åland Islands 	AX 	ALA 	248
"""

# Fixes
MID = re.sub(r'( \(.*?\))', "", MID)
MID = MID \
         .replace("France - Guiana", "French Guiana") \
         .replace("France - Wallis and Futuna Islands", "Wallis and Futuna") \
         .replace("France - Saint Paul and Amsterdam Islands", "French Southern Territories") \
         .replace("France - Kerguelen Islands", "French Southern Territories") \
         .replace("France - Crozet Archipelago", "French Southern Territories") \
         .replace("France - Reunion", "French Southern Territories") \
         .replace("France - Adelie Land", "French Southern Territories") \
         .replace("France - ", "") \
         .replace("Australia - ", "") \
         .replace("New Zealand - ", "") \
         .replace("China - ", "") \
         .replace("Denmark - ", "") \
         .replace("Netherlands - ", "") \
         .replace("United States of America - Alaska", "United States of America") \
         .replace("United States of America - ", "") \
         .replace("United Kingdom of Great Britain and Northern Ireland - Pitcairn Island", 
                  "Pitcairn") \
         .replace("United Kingdom of Great Britain and Northern Ireland - Falkland Islands", 
                  "Falkland Islands [Malvinas]") \
         .replace("United Kingdom of Great Britain and Northern Ireland - Saint Helena", 
                  "Saint Helena, Ascension and Tristan da Cunha") \
         .replace("United Kingdom of Great Britain and Northern Ireland - Ascension Island", 
                  "Saint Helena, Ascension and Tristan da Cunha") \
         .replace("United Kingdom of Great Britain and Northern Ireland - ", "") \
         .replace("Argentine Republic", "Argentina") \
         .replace("Democratic Republic of the Congo", "Congo") \
         .replace("Togolese Republic", "Togo") \
         .replace("Gabonese Republic", "Gabon") \
         .replace("Kyrgyz Republic", "Kyrgyzstan") \
         .replace("Democratic People's Republic of Korea", "Korea") \
         .replace("State of Palestine", "Palestine, State of") \
         .replace("Czech Republic", "Czechia") \
         .replace("Slovak Republic", "Slovakia") \
         .replace("Portugal - Madeira", "Portugal") \
         .replace("Portugal - Azores", "Portugal") \
         .replace("Tanzania", "Tanzania, United Republic of") \
         .replace("North Macedonia", "Republic of North Macedonia")
         
ISO_3166 = ISO_3166.replace("Virgin Islands (U.S.)", "United States Virgin Islands") \
                   .replace("Virgin Islands (British)", "British Virgin Islands") 
ISO_3166 = re.sub(r'( \(.*?\))', "", ISO_3166)
ISO_3166 += "\nFrance - Reunion  RE ZZZ 000"
ISO_3166 += "\nVatican City State  VA ZZZ 000"

# Parse ISO-3166 mapping
country_name_to_iso3166 = {}
for iso3166_line in ISO_3166.split('\n'):
  mo = re.compile(r"^(.*?)\s+([A-Z][A-Z])\s+([A-Z][A-Z][A-Z])\s+([0-9]+)\s*$").match(iso3166_line)
  if mo != None:
    country_name_to_iso3166[mo.group(1)] = mo.group(2)
  elif iso3166_line != "":
    print("Failed to parse: %s" % iso3166_line)

# Parse MMSI MID mapping
prev_mid = None
for mid_line in MID.split('\n'):
  mo = re.compile(r"^(\d\d\d)\s+(.*)$").match(mid_line)
  if mo != None:
    mid = mo.group(1)
    country = mo.group(2)

    if country in country_name_to_iso3166:
      a2 = country_name_to_iso3166[country]
      if mid != prev_mid:
        print("            %s => Some(\"%s\"), // %s" % (mid, a2, country))
      else:
        print("//            %s => Some(\"%s\"), // %s" % (mid, a2, country))
    else:
      print("Unmatching country: %s" % country)
      sys.exit(1)
    prev_mid = mid

