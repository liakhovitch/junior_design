EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr USLetter 11000 8500
encoding utf-8
Sheet 2 3
Title "Egg Timer"
Date ""
Rev "1.0"
Comp "Anton Liakhovitch"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L dk_USB-DVI-HDMI-Connectors:10103594-0001LF J?
U 1 1 60B0663A
P 3297 3288
AR Path="/60B0663A" Ref="J?"  Part="1" 
AR Path="/60AE680C/60B0663A" Ref="J?"  Part="1" 
F 0 "J?" H 3311 4011 50  0000 C CNN
F 1 "10103594-0001LF" H 3311 3920 50  0000 C CNN
F 2 "digikey-footprints:USB_Micro_B_Female_10103594-0001LF" H 3497 3488 60  0001 L CNN
F 3 "https://cdn.amphenol-icc.com/media/wysiwyg/files/drawing/10103594.pdf" H 3497 3588 60  0001 L CNN
F 4 "609-4050-1-ND" H 3497 3688 60  0001 L CNN "Digi-Key_PN"
F 5 "10103594-0001LF" H 3497 3788 60  0001 L CNN "MPN"
F 6 "Connectors, Interconnects" H 3497 3888 60  0001 L CNN "Category"
F 7 "USB, DVI, HDMI Connectors" H 3497 3988 60  0001 L CNN "Family"
F 8 "https://cdn.amphenol-icc.com/media/wysiwyg/files/drawing/10103594.pdf" H 3497 4088 60  0001 L CNN "DK_Datasheet_Link"
F 9 "/product-detail/en/amphenol-icc-fci/10103594-0001LF/609-4050-1-ND/2350357" H 3497 4188 60  0001 L CNN "DK_Detail_Page"
F 10 "CONN RCPT USB2.0 MICRO B SMD R/A" H 3497 4288 60  0001 L CNN "Description"
F 11 "Amphenol ICC (FCI)" H 3497 4388 60  0001 L CNN "Manufacturer"
F 12 "Active" H 3497 4488 60  0001 L CNN "Status"
	1    3297 3288
	1    0    0    -1  
$EndComp
$Comp
L Switch:SW_SPST SW?
U 1 1 60B06640
P 4047 4088
AR Path="/60B06640" Ref="SW?"  Part="1" 
AR Path="/60AE680C/60B06640" Ref="SW?"  Part="1" 
F 0 "SW?" H 4047 4323 50  0000 C CNN
F 1 "PWR_BTN" H 4047 4232 50  0000 C CNN
F 2 "" H 4047 4088 50  0001 C CNN
F 3 "~" H 4047 4088 50  0001 C CNN
	1    4047 4088
	1    0    0    -1  
$EndComp
$Comp
L Device:Battery_Cell BT?
U 1 1 60B06646
P 6247 4438
AR Path="/60B06646" Ref="BT?"  Part="1" 
AR Path="/60AE680C/60B06646" Ref="BT?"  Part="1" 
F 0 "BT?" H 6365 4534 50  0000 L CNN
F 1 "Bat" H 6365 4443 50  0000 L CNN
F 2 "" V 6247 4498 50  0001 C CNN
F 3 "~" V 6247 4498 50  0001 C CNN
	1    6247 4438
	1    0    0    -1  
$EndComp
Wire Wire Line
	6247 4538 6247 4588
Wire Wire Line
	6147 4238 6247 4238
Wire Wire Line
	5447 2688 5447 2438
Wire Wire Line
	3647 2438 3647 3088
Wire Wire Line
	3647 3088 3547 3088
Wire Wire Line
	3647 2438 3897 2438
$Comp
L Device:C C?
U 1 1 60B06652
P 3897 2588
AR Path="/60B06652" Ref="C?"  Part="1" 
AR Path="/60AE680C/60B06652" Ref="C?"  Part="1" 
F 0 "C?" H 4012 2634 50  0000 L CNN
F 1 "10uF" H 4012 2543 50  0000 L CNN
F 2 "" H 3935 2438 50  0001 C CNN
F 3 "~" H 3897 2588 50  0001 C CNN
	1    3897 2588
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 60B06658
P 3897 2738
AR Path="/60B06658" Ref="#PWR?"  Part="1" 
AR Path="/60AE680C/60B06658" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 3897 2488 50  0001 C CNN
F 1 "GND" H 3902 2565 50  0000 C CNN
F 2 "" H 3897 2738 50  0001 C CNN
F 3 "" H 3897 2738 50  0001 C CNN
	1    3897 2738
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small_US R?
U 1 1 60B0665E
P 4547 4288
AR Path="/60B0665E" Ref="R?"  Part="1" 
AR Path="/60AE680C/60B0665E" Ref="R?"  Part="1" 
F 0 "R?" V 4342 4288 50  0000 C CNN
F 1 "400R" V 4433 4288 50  0000 C CNN
F 2 "" H 4547 4288 50  0001 C CNN
F 3 "~" H 4547 4288 50  0001 C CNN
	1    4547 4288
	0    -1   -1   0   
$EndComp
Wire Wire Line
	4647 4288 4747 4288
$Comp
L power:GND #PWR?
U 1 1 60B06665
P 3797 3588
AR Path="/60B06665" Ref="#PWR?"  Part="1" 
AR Path="/60AE680C/60B06665" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 3797 3338 50  0001 C CNN
F 1 "GND" H 3802 3415 50  0000 C CNN
F 2 "" H 3797 3588 50  0001 C CNN
F 3 "" H 3797 3588 50  0001 C CNN
	1    3797 3588
	1    0    0    -1  
$EndComp
Wire Wire Line
	3547 3488 3797 3488
Wire Wire Line
	3797 3488 3797 3588
$Comp
L Device:R_Small_US R?
U 1 1 60B0666D
P 6497 3138
AR Path="/60B0666D" Ref="R?"  Part="1" 
AR Path="/60AE680C/60B0666D" Ref="R?"  Part="1" 
F 0 "R?" V 6292 3138 50  0000 C CNN
F 1 "160R" V 6383 3138 50  0000 C CNN
F 2 "" H 6497 3138 50  0001 C CNN
F 3 "~" H 6497 3138 50  0001 C CNN
	1    6497 3138
	0    1    1    0   
$EndComp
Wire Wire Line
	4247 4088 4747 4088
Wire Wire Line
	6597 3138 6647 3138
Wire Wire Line
	6647 3138 6647 3188
$Comp
L timer_custom:LTC3553 U?
U 1 1 60B06676
P 5447 3588
AR Path="/60B06676" Ref="U?"  Part="1" 
AR Path="/60AE680C/60B06676" Ref="U?"  Part="1" 
F 0 "U?" H 5447 4669 50  0000 C CNN
F 1 "LTC3553" H 5447 4578 50  0000 C CNN
F 2 "Package_DFN_QFN:QFN-20-1EP_3x3mm_P0.4mm_EP1.65x1.65mm" H 5447 4088 50  0001 C CNN
F 3 "https://www.analog.com/media/en/technical-documentation/data-sheets/3553fc.pdf" H 5447 3538 50  0001 C CNN
	1    5447 3588
	1    0    0    -1  
$EndComp
$Comp
L Device:L L?
U 1 1 60B0667C
P 6547 3688
AR Path="/60B0667C" Ref="L?"  Part="1" 
AR Path="/60AE680C/60B0667C" Ref="L?"  Part="1" 
F 0 "L?" V 6366 3688 50  0000 C CNN
F 1 "4.7uH" V 6457 3688 50  0000 C CNN
F 2 "" H 6547 3688 50  0001 C CNN
F 3 "~" H 6547 3688 50  0001 C CNN
	1    6547 3688
	0    1    1    0   
$EndComp
$Comp
L Device:C C?
U 1 1 60B06682
P 7497 3838
AR Path="/60B06682" Ref="C?"  Part="1" 
AR Path="/60AE680C/60B06682" Ref="C?"  Part="1" 
F 0 "C?" H 7612 3884 50  0000 L CNN
F 1 "10uF" H 7612 3793 50  0000 L CNN
F 2 "" H 7535 3688 50  0001 C CNN
F 3 "~" H 7497 3838 50  0001 C CNN
	1    7497 3838
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 60B06688
P 6747 3888
AR Path="/60B06688" Ref="C?"  Part="1" 
AR Path="/60AE680C/60B06688" Ref="C?"  Part="1" 
F 0 "C?" H 6862 3934 50  0000 L CNN
F 1 "10pF" H 6862 3843 50  0000 L CNN
F 2 "" H 6785 3738 50  0001 C CNN
F 3 "~" H 6747 3888 50  0001 C CNN
	1    6747 3888
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small_US R?
U 1 1 60B0668E
P 7097 3838
AR Path="/60B0668E" Ref="R?"  Part="1" 
AR Path="/60AE680C/60B0668E" Ref="R?"  Part="1" 
F 0 "R?" H 7165 3884 50  0000 L CNN
F 1 "250k" H 7165 3793 50  0000 L CNN
F 2 "" H 7097 3838 50  0001 C CNN
F 3 "~" H 7097 3838 50  0001 C CNN
	1    7097 3838
	1    0    0    -1  
$EndComp
$Comp
L Device:R_Small_US R?
U 1 1 60B06694
P 7247 4038
AR Path="/60B06694" Ref="R?"  Part="1" 
AR Path="/60AE680C/60B06694" Ref="R?"  Part="1" 
F 0 "R?" V 7042 4038 50  0000 C CNN
F 1 "80K" V 7133 4038 50  0000 C CNN
F 2 "" H 7247 4038 50  0001 C CNN
F 3 "~" H 7247 4038 50  0001 C CNN
	1    7247 4038
	0    -1   -1   0   
$EndComp
Wire Wire Line
	6697 3688 6747 3688
Wire Wire Line
	6747 3738 6747 3688
Connection ~ 6747 3688
Wire Wire Line
	7097 3938 7097 4038
Wire Wire Line
	7097 4038 7147 4038
Wire Wire Line
	7097 4038 6747 4038
Connection ~ 7097 4038
Wire Wire Line
	6747 3688 7097 3688
Wire Wire Line
	7097 3738 7097 3688
Connection ~ 6747 4038
Wire Wire Line
	6297 4038 6747 4038
Wire Wire Line
	7497 3988 7497 4038
Wire Wire Line
	7497 3688 7097 3688
Connection ~ 7097 3688
Wire Wire Line
	7497 4038 7347 4038
Wire Wire Line
	6147 3538 6297 3538
Wire Wire Line
	6397 3438 6147 3438
Wire Wire Line
	7497 3688 7747 3688
Connection ~ 7497 3688
Text GLabel 4347 2888 0    50   Input ~ 0
vout
$Comp
L power:GND #PWR?
U 1 1 60B066AF
P 4447 3738
AR Path="/60B066AF" Ref="#PWR?"  Part="1" 
AR Path="/60AE680C/60B066AF" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 4447 3488 50  0001 C CNN
F 1 "GND" H 4452 3565 50  0000 C CNN
F 2 "" H 4447 3738 50  0001 C CNN
F 3 "" H 4447 3738 50  0001 C CNN
	1    4447 3738
	1    0    0    -1  
$EndComp
Wire Wire Line
	4747 2888 4447 2888
Wire Wire Line
	4447 3738 4447 3688
Wire Wire Line
	4597 3688 4747 3688
Wire Wire Line
	4747 3188 4597 3188
Connection ~ 4597 3188
Wire Wire Line
	4597 3188 4597 3388
Wire Wire Line
	4747 3388 4597 3388
Connection ~ 4597 3388
Wire Wire Line
	4597 3388 4597 3488
Wire Wire Line
	4747 3288 4447 3288
Wire Wire Line
	4447 3288 4447 2888
Connection ~ 4447 2888
Wire Wire Line
	4447 2888 4347 2888
Wire Wire Line
	4747 3488 4597 3488
Connection ~ 4597 3488
Wire Wire Line
	4597 3488 4597 3688
Wire Wire Line
	4747 2988 4597 2988
Wire Wire Line
	4597 2988 4597 3188
Wire Wire Line
	3897 2438 5447 2438
Connection ~ 3897 2438
Wire Wire Line
	7497 4588 6247 4588
Wire Wire Line
	6247 4588 5447 4588
Wire Wire Line
	5447 4588 5447 4488
Connection ~ 6247 4588
Wire Wire Line
	4297 4288 4297 4588
Connection ~ 5447 4588
Wire Wire Line
	3847 4088 3847 4588
Connection ~ 4297 4588
Wire Wire Line
	4297 4588 3847 4588
Wire Wire Line
	4297 4588 5447 4588
Wire Wire Line
	4297 4288 4447 4288
Text Notes 5047 2238 0    200  ~ 0
Power
$Comp
L power:GND #PWR?
U 1 1 60B066D5
P 5447 4688
AR Path="/60B066D5" Ref="#PWR?"  Part="1" 
AR Path="/60AE680C/60B066D5" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 5447 4438 50  0001 C CNN
F 1 "GND" H 5452 4515 50  0000 C CNN
F 2 "" H 5447 4688 50  0001 C CNN
F 3 "" H 5447 4688 50  0001 C CNN
	1    5447 4688
	1    0    0    -1  
$EndComp
Wire Wire Line
	5447 4688 5447 4588
Wire Wire Line
	4447 3688 4597 3688
Connection ~ 4597 3688
Text Notes 4397 2788 0    50   ~ 0
Settings
Text Notes 6747 3588 0    50   ~ 0
Buck Regulator Feedback
Wire Wire Line
	6397 3438 6397 3688
Wire Wire Line
	6297 3538 6297 4038
Wire Wire Line
	7497 4038 7497 4588
Connection ~ 7497 4038
Text Notes 6997 2788 0    50   ~ 0
Raw Output
Text GLabel 7747 2888 2    50   Input ~ 0
vout
$Comp
L power:GND #PWR?
U 1 1 60B066E6
P 7497 3188
AR Path="/60B066E6" Ref="#PWR?"  Part="1" 
AR Path="/60AE680C/60B066E6" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 7497 2938 50  0001 C CNN
F 1 "GND" H 7502 3015 50  0000 C CNN
F 2 "" H 7497 3188 50  0001 C CNN
F 3 "" H 7497 3188 50  0001 C CNN
	1    7497 3188
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 60B066EC
P 7497 3038
AR Path="/60B066EC" Ref="C?"  Part="1" 
AR Path="/60AE680C/60B066EC" Ref="C?"  Part="1" 
F 0 "C?" H 7612 3084 50  0000 L CNN
F 1 "10uF" H 7612 2993 50  0000 L CNN
F 2 "" H 7535 2888 50  0001 C CNN
F 3 "~" H 7497 3038 50  0001 C CNN
	1    7497 3038
	1    0    0    -1  
$EndComp
Connection ~ 6747 2888
Wire Wire Line
	7097 3338 7097 2888
Wire Wire Line
	6647 3188 6747 3188
$Comp
L Device:LED D?
U 1 1 60B066F5
P 6747 3038
AR Path="/60B066F5" Ref="D?"  Part="1" 
AR Path="/60AE680C/60B066F5" Ref="D?"  Part="1" 
F 0 "D?" V 6786 2920 50  0000 R CNN
F 1 "chg" V 6695 2920 50  0000 R CNN
F 2 "" H 6747 3038 50  0001 C CNN
F 3 "~" H 6747 3038 50  0001 C CNN
	1    6747 3038
	0    -1   -1   0   
$EndComp
Connection ~ 7497 2888
Wire Wire Line
	7497 2888 7747 2888
Connection ~ 7097 2888
Wire Wire Line
	7097 2888 7497 2888
Wire Wire Line
	6747 2888 7097 2888
Wire Wire Line
	6147 3338 7097 3338
Wire Wire Line
	6147 2888 6747 2888
Wire Wire Line
	6397 3138 6147 3138
Text Notes 6197 4188 0    50   ~ 0
Battery
Text Notes 4147 4738 0    50   ~ 0
Charge Current Select
Text HLabel 7747 3688 2    50   Input ~ 0
3v3
$EndSCHEMATC
