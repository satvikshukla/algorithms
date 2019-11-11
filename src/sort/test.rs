mod test {
    use crate::sort::{bubble_sort, selection_sort, merge_sort,
                insertion_sort, quick_sort, heap_sort, tim_sort};

    fn test_empty_arr<F: Fn(&mut [u8])>(f: F) {
        let mut arr = vec![];
        f(&mut arr);
        let expected: Vec<u8> = vec![];
        assert_eq!(arr, expected);
    }

    fn test_one_element_arr<F: Fn(&mut [u8])>(f: F) {
        let mut arr = vec![2];
        f(&mut arr);
        assert_eq!(arr, vec![2]);
    }

    fn test_pre_sorted_arr<F: Fn(&mut [i32])>(f: F) {
        let mut arr = vec![-4, 9, 11];
        f(&mut arr);
        assert_eq!(arr, vec![-4, 9, 11]);
    }

    fn test_int_arr<F: Fn(&mut [i32])>(f: F) {
        let mut arr = vec![5, -9, 4, 1, 3, 3];
        f(&mut arr);
        assert_eq!(arr, vec![-9, 1, 3, 3, 4, 5]);
    }

    fn test_float_arr<F: Fn(&mut [f32])>(f: F) {
        let mut arr = vec![-1.1, -7.5, -0.1, 1.1, 1.11];
        f(&mut arr);
        assert_eq!(arr, vec![-7.5, -1.1, -0.1, 1.1, 1.11]);
    }

    fn test_string_arr<F: Fn(&mut [String])>(f: F) {
        let mut arr = vec![String::from("abc"), String::from("aa"), String::from("a"), String::from("z"), String::from("abb")];
        f(&mut arr);
        assert_eq!(arr, vec!["a", "aa", "abb", "abc", "z"]);
    }

    fn test_long_int_arr<F: Fn(&mut [i32])>(f: F) {
        let mut arr = vec![8254, 4285, -7732, 4631, -837, -7863, 9926, 9960, 8116, -5469, 1775, -1090, -6885, -7256, -1380, -3108, -9015, -5728, 5288, -2248, -7262, 9613, -3880, 4234, -9317, 2399, -692, 1488, 1331, 4428, 3182, 4183, -1624, 5089, -6145, -3310, -3155, -6279, 7201, 8949, -4470, -3709, 7493, -8483, -7212, 351, -7862, -8430, 5804, 6539, 269, 8219, 3383, -4838, 1937, -3279, 1415, -9847, 7822, -1709, -712, -3024, 2141, 8753, -6839, -8629, -5960, -8598, 443, -6647, -6667, -8638, 3725, -297, -4540, 6212, -1970, -9475, -6709, -5293, -7387, -5034, 3471, -7221, -8366, 5336, -5967, -4544, 2945, 1323, 1575, -9501, 9557, -3162, -4671, -2805, 3424, -9423, -3851, 9976, -8708, 2425, -7968, -7651, 2032, -3441, -9116, 7512, 729, -8221, -8788, 5781, 8166, 5722, -5222, 1468, -2199, -8364, -7219, 749, -4766, 2448, -1555, -8555, -3205, 8861, 776, 6375, 632, -5406, -2709, 1966, 9266, -159, 5848, 8665, -950, 7884, -920, -3564, -6795, 3466, -8905, -9635, 2823, 9247, 8128, -9417, 6699, 4375, 9718, 2042, 7788, -8572, 297, -5810, -8980, 3932, 1207, -2559, 6031, 1809, 9379, 3192, -2182, 5806, -5738, -6347, 2199, -1213, 2363, 11, -1691, -823, 99, -6065, -2531, 3639, 1911, -334, -6190, -992, 3171, -5270, -4717, 9298, -2753, -3632, -2998, 925, -3667, 7819, -4033, 9931, -6708, -7920, 3397, -7067, -9480, -6566, 8504, 8905, 3552, -9606, 1032, -7905, -3353, 8819, -5150, -8194, 993, -5530, -6290, -7158, 6957, 3442, -8597, -5383, 2347, 7950, -8447, 8713, -8, 6933, 1300, -5143, 3318, -2512, -1888, 5106, -3689, 7405, 9998, 9444, -3459, 7663, 8215, 8628, 3206, 7041, 3715, -3191, -2929, 6932, -1790, -8665, 4201, -9149, 5487, -606, 8720, -8417, 9257, -2091, 2414, 7338, -7418, -3620, -2980, -4450, 2632, -1514, 8692, 8756, 6355, -1505, 9034, -6101, -8561, -1492, -1036, 3450, 2581, 6150, 9806, 3222, 8381, -4199, 4931, 3628, -3648, -7055, 8889, -5729, 2765, -6248, 4190, -3142, 7728, 3183, 8974, -6768, -941, -7912, 4794, -74, -9680, -7372, 8125, -6000, 6288, 3950, 9940, 5954, 147, -8112, 2882, 2016, -1092, -7838, 8471, -763, 5312, -8104, -8134, 6643, 9852, -6316, -7774, 534, 7178, 8246, 7943, -2282, 8908, 7639, -5377, -8937, 1375, -8915, 3692, -4892, -9835, -9718, 1868, 6923, 8126, -6046, -6873, 3603, -1799, -3957, 5858, -3930, -6036, -7945, -5813, 681, -3150, -9695, 843, 2255, -6807, 6515, -6423, -2070, 6096, 3568, 6964, -8534, 3651, -9068, -9396, -5623, -2388, -9554, 2456, -6246, 1504, -7977, 2480, 7089, -7331, 1659, 2495, -6355, -4832, 7991, -4423, 6329, 2869, -4460, 3813, 7480, 426, -1835, 952, 7804, -8855, 9651, -6599, 1001, 5937, -5245, 7528, 4521, 5135, -4895, 4974, -7996, -8891, -3092, -3414, 6859, -8107, 8329, -6482, 6772, -3119, -4077, -3595, 7315, -9453, 7150, -570, 972, 9774, 1505, 3918, 4036, 5100, -7200, -3369, -9816, -7803, 2865, -8322, 389, -580, -196, -9711, -3122, 504, 7236, -4773, -901, 2851, -4703, 1337, -2729, -4742, -5358, -1563, -97, 9829, -8465, 3248, 8261, -7049, -3491, 4382, -4630, 1509, -6882, -4085, -6954, -8245, 720, 8545, -4758, -9587, 3085, 1198, 4185, 7409, -522, -5000, 4379, -5413, 3691, -1927, 9441, 1795, 5176, -6428, 699, -2120, -3312, 6536, 1947, -533, 1438, -7115, 2345, 9879, -9818, 2836, 8611, 1694, -5544, 9290, -5014, -5446, -7140, 6343, 8206, -5966, -1685, 5226, 3063, 6032, -3704, 4363, 5962, 7968, -7956, -9479, 7076, 9423, -9232, 5322, -809, 2187, 3237, 2256, 5116, -5866, -7420, -3535, -40, 5846, -4992, 2226, 7837, 132, -6110, -3991, 2122, -1787, 7541, -30, -1285, 7557, -3374, 6419, 9320, 4552, 4629, -6083, 4544, 5510, -6335, 9121, -5773, 5951, 4476, -7340, 5870, -4611, 8386, 8904, -1697, -8868, 975, 2817, -5104, 8220, -2535, -9393, -495, -2317, -1689, 9732, -5667, 433, -4453, 2031, 9797, -5181, -9143, -250, -4184, 3229, -1444, -6597, -9831, 5165, 1065, 1767, 4796, 6896, -3431, 3869, 7346, 9312, 8327, -8556, 627, 77, -3643, -8512, -614, -5140, 5124, -9122, -2543, -4040, 1655, -1391, -3784, 4331, -7083, -2007, -934, 1017, -7311, 279, 225, 4128, 2831, -4899, 9579, -4755, -7832, -3593, 6348, -7725, 3297, 1398, -9048, -1997, -9539, -5393, -2882, -2154, -1783, 5358, 4900, 1972, -4770, 8785, -741, 5794, 4991, -8728, 9991, -1385, -1024, 6081, -5554, 6582, 4775, 9880, 6570, -6003, 9028, 6246, -8270, 3200, -4297, 5043, -3850, -6876, 5641, -9159, -8702, 3122, 6272, -5371, 3586, -6371, 3757, -819, 6506, 4101, -4783, -9299, -4389, -8000, -1171, -1988, 4750, 5162, 1303, 7769, 4441, 5239, -8303, -1762, -5067, 6069, -8647, -2315, -3849, -358, -6627, -2340, 3573, -1105, -4650, 6534, 7894, 4912, 7709, 6465, -8399, 5620, -9223, -6539, -9692, -9549, -5776, 3539, -5203, -9347, 5490, 5455, 117, 165, 2233, 4181, 1820, 8242, -5802, 96, -3721, 7866, 358, -8764, -2293, 9451, 3904, 585, 7206, 6496, 5784, 5161, -11, 2281, -5312, -7808, -5880, -5351, -3375, -2607, 3084, -405, -8865, -7306, -2567, -5117, -6280, 871, -6545, -721, -2962, 2504, -6513, -3144, 9187, -504, 4300, -5362, -6374, 9062, -2435, 3404, -2480, -6639, 7121, -3693, -5328, 8513, -952, 2642, 1839, 4357, 4528, -1159, 1564, 1051, 2937, 5977, -9289, -7930, -5803, -820, -4360, 2500, 7418, 3038, -216, 8968, -3806, 1427, -4844, -710, -2126, 7003, 1000, -645, 955, 2561, 5607, -5761, 1, -9348, 7462, -8320, -8036, 6759, -5522, 9846, -2257, -733, -6592, 9354, 983, 698, -8237, 5252, -3193, 1622, -1961, -4473, -3878, -5159, -8029, -1100, -5650, -7285, 1678, -7188, 6426, 5648, 6580, 6585, 3234, 4643, -2735, 1205, -7134, -4382, -4102, -5456, -528, -80, 9938, 8750, -3222, -7120, 9833, -4619, -2207, 7190, 8704, 9016, -6343, 6214, -922, 91, 3455, -9489, -3413, 657, 7217, -6745, 7377, -7936, -3622, -7354, 6749, 466, 1211, 1727, -9582, 6619, -6092, -2886, 129, 1195, -4474, 259, 6392, 8911, 364, -1115, -6769, -5049, 1286, -7672, -38, 2015, 3402, 2550, -6051, -6687, -3372, 1428, 2071, -1980, -3232, -4214, -6028, -1333, -7902, -4578, 3497, 3258, 387, 5466, 217, 9024, -5442, -9315, 5289, -2800, -2863, -9002, 148, 8052, 342, 2441, 7537, -9868, -8633, 3930, -1057, 4932, -7023, -8267, 1271, -7386, 2886, -7097, 9100, -2708, -1839, -3063, 8321, 9597, 5504, 2350, -8379, -452, -7554, 1476, 9384, -4260, -4375, 2174, 5719, 8608, 9515, -9050, 4289, -8654, -6401, 5299, 605, -9221, -5153, 6066, -2923, -1296, -1028, -8086, -4723, 9524, -582, -4179, -8960, 9808, -5572, -6867, -2620, 4687, 3548, 162, 8932, -3212, -5, 6511, 5717, 5365, -5061, 5213, -2795, 9471, -3557, 6819, -9762, 5626, 4134, -3847, 4367, 68, -1043, -6736, 7715, -6518, -6841, 6573, -501, 9448, 2546, 7686, -5360, 9391, -6830];
        f(&mut arr);
        assert_eq!(arr, vec![-9868, -9847, -9835, -9831, -9818, -9816, -9762, -9718, -9711, -9695, -9692, -9680, -9635, -9606, -9587, -9582, -9554, -9549, -9539, -9501, -9489, -9480, -9479, -9475, -9453, -9423, -9417, -9396, -9393, -9348, -9347, -9317, -9315, -9299, -9289, -9232, -9223, -9221, -9159, -9149, -9143, -9122, -9116, -9068, -9050, -9048, -9015, -9002, -8980, -8960, -8937, -8915, -8905, -8891, -8868, -8865, -8855, -8788, -8764, -8728, -8708, -8702, -8665, -8654, -8647, -8638, -8633, -8629, -8598, -8597, -8572, -8561, -8556, -8555, -8534, -8512, -8483, -8465, -8447, -8430, -8417, -8399, -8379, -8366, -8364, -8322, -8320, -8303, -8270, -8267, -8245, -8237, -8221, -8194, -8134, -8112, -8107, -8104, -8086, -8036, -8029, -8000, -7996, -7977, -7968, -7956, -7945, -7936, -7930, -7920, -7912, -7905, -7902, -7863, -7862, -7838, -7832, -7808, -7803, -7774, -7732, -7725, -7672, -7651, -7554, -7420, -7418, -7387, -7386, -7372, -7354, -7340, -7331, -7311, -7306, -7285, -7262, -7256, -7221, -7219, -7212, -7200, -7188, -7158, -7140, -7134, -7120, -7115, -7097, -7083, -7067, -7055, -7049, -7023, -6954, -6885, -6882, -6876, -6873, -6867, -6841, -6839, -6830, -6807, -6795, -6769, -6768, -6745, -6736, -6709, -6708, -6687, -6667, -6647, -6639, -6627, -6599, -6597, -6592, -6566, -6545, -6539, -6518, -6513, -6482, -6428, -6423, -6401, -6374, -6371, -6355, -6347, -6343, -6335, -6316, -6290, -6280, -6279, -6248, -6246, -6190, -6145, -6110, -6101, -6092, -6083, -6065, -6051, -6046, -6036, -6028, -6003, -6000, -5967, -5966, -5960, -5880, -5866, -5813, -5810, -5803, -5802, -5776, -5773, -5761, -5738, -5729, -5728, -5667, -5650, -5623, -5572, -5554, -5544, -5530, -5522, -5469, -5456, -5446, -5442, -5413, -5406, -5393, -5383, -5377, -5371, -5362, -5360, -5358, -5351, -5328, -5312, -5293, -5270, -5245, -5222, -5203, -5181, -5159, -5153, -5150, -5143, -5140, -5117, -5104, -5067, -5061, -5049, -5034, -5014, -5000, -4992, -4899, -4895, -4892, -4844, -4838, -4832, -4783, -4773, -4770, -4766, -4758, -4755, -4742, -4723, -4717, -4703, -4671, -4650, -4630, -4619, -4611, -4578, -4544, -4540, -4474, -4473, -4470, -4460, -4453, -4450, -4423, -4389, -4382, -4375, -4360, -4297, -4260, -4214, -4199, -4184, -4179, -4102, -4085, -4077, -4040, -4033, -3991, -3957, -3930, -3880, -3878, -3851, -3850, -3849, -3847, -3806, -3784, -3721, -3709, -3704, -3693, -3689, -3667, -3648, -3643, -3632, -3622, -3620, -3595, -3593, -3564, -3557, -3535, -3491, -3459, -3441, -3431, -3414, -3413, -3375, -3374, -3372, -3369, -3353, -3312, -3310, -3279, -3232, -3222, -3212, -3205, -3193, -3191, -3162, -3155, -3150, -3144, -3142, -3122, -3119, -3108, -3092, -3063, -3024, -2998, -2980, -2962, -2929, -2923, -2886, -2882, -2863, -2805, -2800, -2795, -2753, -2735, -2729, -2709, -2708, -2620, -2607, -2567, -2559, -2543, -2535, -2531, -2512, -2480, -2435, -2388, -2340, -2317, -2315, -2293, -2282, -2257, -2248, -2207, -2199, -2182, -2154, -2126, -2120, -2091, -2070, -2007, -1997, -1988, -1980, -1970, -1961, -1927, -1888, -1839, -1835, -1799, -1790, -1787, -1783, -1762, -1709, -1697, -1691, -1689, -1685, -1624, -1563, -1555, -1514, -1505, -1492, -1444, -1391, -1385, -1380, -1333, -1296, -1285, -1213, -1171, -1159, -1115, -1105, -1100, -1092, -1090, -1057, -1043, -1036, -1028, -1024, -992, -952, -950, -941, -934, -922, -920, -901, -837, -823, -820, -819, -809, -763, -741, -733, -721, -712, -710, -692, -645, -614, -606, -582, -580, -570, -533, -528, -522, -504, -501, -495, -452, -405, -358, -334, -297, -250, -216, -196, -159, -97, -80, -74, -40, -38, -30, -11, -8, -5, 1, 11, 68, 77, 91, 96, 99, 117, 129, 132, 147, 148, 162, 165, 217, 225, 259, 269, 279, 297, 342, 351, 358, 364, 387, 389, 426, 433, 443, 466, 504, 534, 585, 605, 627, 632, 657, 681, 698, 699, 720, 729, 749, 776, 843, 871, 925, 952, 955, 972, 975, 983, 993, 1000, 1001, 1017, 1032, 1051, 1065, 1195, 1198, 1205, 1207, 1211, 1271, 1286, 1300, 1303, 1323, 1331, 1337, 1375, 1398, 1415, 1427, 1428, 1438, 1468, 1476, 1488, 1504, 1505, 1509, 1564, 1575, 1622, 1655, 1659, 1678, 1694, 1727, 1767, 1775, 1795, 1809, 1820, 1839, 1868, 1911, 1937, 1947, 1966, 1972, 2015, 2016, 2031, 2032, 2042, 2071, 2122, 2141, 2174, 2187, 2199, 2226, 2233, 2255, 2256, 2281, 2345, 2347, 2350, 2363, 2399, 2414, 2425, 2441, 2448, 2456, 2480, 2495, 2500, 2504, 2546, 2550, 2561, 2581, 2632, 2642, 2765, 2817, 2823, 2831, 2836, 2851, 2865, 2869, 2882, 2886, 2937, 2945, 3038, 3063, 3084, 3085, 3122, 3171, 3182, 3183, 3192, 3200, 3206, 3222, 3229, 3234, 3237, 3248, 3258, 3297, 3318, 3383, 3397, 3402, 3404, 3424, 3442, 3450, 3455, 3466, 3471, 3497, 3539, 3548, 3552, 3568, 3573, 3586, 3603, 3628, 3639, 3651, 3691, 3692, 3715, 3725, 3757, 3813, 3869, 3904, 3918, 3930, 3932, 3950, 4036, 4101, 4128, 4134, 4181, 4183, 4185, 4190, 4201, 4234, 4285, 4289, 4300, 4331, 4357, 4363, 4367, 4375, 4379, 4382, 4428, 4441, 4476, 4521, 4528, 4544, 4552, 4629, 4631, 4643, 4687, 4750, 4775, 4794, 4796, 4900, 4912, 4931, 4932, 4974, 4991, 5043, 5089, 5100, 5106, 5116, 5124, 5135, 5161, 5162, 5165, 5176, 5213, 5226, 5239, 5252, 5288, 5289, 5299, 5312, 5322, 5336, 5358, 5365, 5455, 5466, 5487, 5490, 5504, 5510, 5607, 5620, 5626, 5641, 5648, 5717, 5719, 5722, 5781, 5784, 5794, 5804, 5806, 5846, 5848, 5858, 5870, 5937, 5951, 5954, 5962, 5977, 6031, 6032, 6066, 6069, 6081, 6096, 6150, 6212, 6214, 6246, 6272, 6288, 6329, 6343, 6348, 6355, 6375, 6392, 6419, 6426, 6465, 6496, 6506, 6511, 6515, 6534, 6536, 6539, 6570, 6573, 6580, 6582, 6585, 6619, 6643, 6699, 6749, 6759, 6772, 6819, 6859, 6896, 6923, 6932, 6933, 6957, 6964, 7003, 7041, 7076, 7089, 7121, 7150, 7178, 7190, 7201, 7206, 7217, 7236, 7315, 7338, 7346, 7377, 7405, 7409, 7418, 7462, 7480, 7493, 7512, 7528, 7537, 7541, 7557, 7639, 7663, 7686, 7709, 7715, 7728, 7769, 7788, 7804, 7819, 7822, 7837, 7866, 7884, 7894, 7943, 7950, 7968, 7991, 8052, 8116, 8125, 8126, 8128, 8166, 8206, 8215, 8219, 8220, 8242, 8246, 8254, 8261, 8321, 8327, 8329, 8381, 8386, 8471, 8504, 8513, 8545, 8608, 8611, 8628, 8665, 8692, 8704, 8713, 8720, 8750, 8753, 8756, 8785, 8819, 8861, 8889, 8904, 8905, 8908, 8911, 8932, 8949, 8968, 8974, 9016, 9024, 9028, 9034, 9062, 9100, 9121, 9187, 9247, 9257, 9266, 9290, 9298, 9312, 9320, 9354, 9379, 9384, 9391, 9423, 9441, 9444, 9448, 9451, 9471, 9515, 9524, 9557, 9579, 9597, 9613, 9651, 9718, 9732, 9774, 9797, 9806, 9808, 9829, 9833, 9846, 9852, 9879, 9880, 9926, 9931, 9938, 9940, 9960, 9976, 9991, 9998]);
    }

    #[test]
    fn selection_sort_empty() {
        test_empty_arr(selection_sort);
    }

    #[test]
    fn selection_sort_one_element() {
        test_one_element_arr(selection_sort);
    }

    #[test]
    fn selection_sort_pre_sorted() {
        test_pre_sorted_arr(selection_sort);
    }

    #[test]
    fn selection_sort_int_sort() {
        test_int_arr(selection_sort);
    }

    #[test]
    fn selection_sort_float_sort() {
        test_float_arr(selection_sort);
    }

    #[test]
    fn selection_sort_string_sort() {
        test_string_arr(selection_sort);
    }

    #[test]
    fn selection_sort_long_int_sort() {
        test_long_int_arr(selection_sort);
    }

    #[test]
    fn bubble_sort_empty() {
        test_empty_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_one_element() {
        test_one_element_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_pre_sorted() {
        test_pre_sorted_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_int_sort() {
        test_int_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_float_sort() {
        test_float_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_string_sort() {
        test_string_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_long_int_sort() {
        test_long_int_arr(bubble_sort);
    }

    #[test]
    fn merge_sort_empty() {
        test_empty_arr(merge_sort);
    }

    #[test]
    fn merge_sort_one_element() {
        test_one_element_arr(merge_sort);
    }

    #[test]
    fn merge_sort_pre_sorted() {
        test_pre_sorted_arr(merge_sort);
    }

    #[test]
    fn merge_sort_int_sort() {
        test_int_arr(merge_sort);
    }

    #[test]
    fn merge_sort_float_sort() {
        test_float_arr(merge_sort);
    }

    // #[test]
    // fn merge_sort_string_sort() {
    //     test_string_arr(merge_sort);
    // }

    #[test]
    fn merge_sort_long_int_sort() {
        test_long_int_arr(merge_sort);
    }

    #[test]
    fn insertion_sort_empty() {
        test_empty_arr(insertion_sort);
    }

    #[test]
    fn insertion_sort_one_element() {
        test_one_element_arr(insertion_sort);
    }

    #[test]
    fn insertion_sort_pre_sorted() {
        test_pre_sorted_arr(insertion_sort);
    }

    #[test]
    fn insertion_sort_int_sort() {
        test_int_arr(insertion_sort);
    }

    #[test]
    fn insertion_sort_float_sort() {
        test_float_arr(insertion_sort);
    }

    // #[test]
    // fn insertion_sort_string_sort() {
    //     test_string_arr(insertion_sort);
    // }

    #[test]
    fn insertion_sort_long_int_sort() {
        test_long_int_arr(insertion_sort);
    }

    #[test]
    fn quick_sort_empty() {
        test_empty_arr(quick_sort);
    }

    #[test]
    fn quick_sort_one_element() {
        test_one_element_arr(quick_sort);
    }

    #[test]
    fn quick_sort_pre_sorted() {
        test_pre_sorted_arr(quick_sort);
    }

    #[test]
    fn quick_sort_int_sort() {
        test_int_arr(quick_sort);
    }

    #[test]
    fn quick_sort_float_sort() {
        test_float_arr(quick_sort);
    }

    // #[test]
    // fn quick_sort_string_sort() {
    //     test_string_arr(quick_sort);
    // }

    #[test]
    fn quick_sort_long_int_sort() {
        test_long_int_arr(quick_sort);
    }

    #[test]
    fn heap_sort_empty() {
        test_empty_arr(heap_sort);
    }

    #[test]
    fn heap_sort_one_element() {
        test_one_element_arr(heap_sort);
    }

    #[test]
    fn heap_sort_pre_sorted() {
        test_pre_sorted_arr(heap_sort);
    }

    #[test]
    fn heap_sort_int_sort() {
        test_int_arr(heap_sort);
    }

    // #[test]
    // fn heap_sort_float_sort() {
    //     test_float_arr(heap_sort);
    // }

    #[test]
    fn heap_sort_string_sort() {
        test_string_arr(heap_sort);
    }

    #[test]
    fn heap_sort_long_int_sort() {
        test_long_int_arr(heap_sort);
    }

    #[test]
    fn tim_sort_empty() {
        test_empty_arr(tim_sort);
    }

    #[test]
    fn tim_sort_one_element() {
        test_one_element_arr(tim_sort);
    }

    #[test]
    fn tim_sort_pre_sorted() {
        test_pre_sorted_arr(tim_sort);
    }

    #[test]
    fn tim_sort_int_sort() {
        test_int_arr(tim_sort);
    }

    #[test]
    fn tim_sort_float_sort() {
        test_float_arr(tim_sort);
    }

    // #[test]
    // fn tim_sort_string_sort() {
    //     test_string_arr(tim_sort);
    // }

    #[test]
    fn tim_sort_long_int_sort() {
        test_long_int_arr(tim_sort);
    }
}
