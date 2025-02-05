/// SPDX-FileCopyrightText: (C) 2024 metricexpansion
/// SPDX-License-Identifier: MIT OR CC-BY-NC-SA-4.0
///
/// See: https://gitlab.com/metricexpansion/SkyrimOutfitSystemSE/-/issues/2#note_2332635556
use crate::bindings::root::SKSE;

impl Default for SKSE::PluginVersionData {
    fn default() -> Self {
        use crate::bindings::root::__BindgenBitfieldUnit;

        Self {
            dataVersion: SKSE::PluginVersionData_kVersion as u32,
            pluginVersion: 0,
            pluginName: [0; 256],
            author: [0; 256],
            supportEmail: [0; 252],
            _bitfield_align_1: [0; 0],
            _bitfield_1: __BindgenBitfieldUnit::<[u8; 1]>::new([0]),
            padding2: 0,
            padding3: 0,
            _bitfield_align_2: [0; 0],
            _bitfield_2: __BindgenBitfieldUnit::<[u8; 1]>::new([0]),
            padding5: 0,
            padding6: 0,
            compatibleVersions: [0; 16],
            xseMinimum: 0,
        }
    }
}

/// - ref: vcpkg_installed\x64-windows\commonlibsse_ng\include\SKSE\Trampoline.h
impl SKSE::Trampoline {
    pub const unsafe fn write_branch<const N: usize>(a_src: usize, a_dst: usize) -> usize {
        let data: u8 = match N {
            5 => 0xE9, // JMP rel32
            6 => 0x25, // JMP r/m64
            _ => panic!("invalid branch size"),
        };

        Self::write_branch_with_data::<N>(a_src, a_dst, data)
    }

    pub const unsafe fn write_branch_with_data<const N: usize>(
        a_src: usize,
        a_dst: usize,
        opcode: u8,
    ) -> usize {
        const { assert!(N == 5 || N == 6) };

        use std::ptr::write_unaligned;

        let src_ptr = a_src as *mut u8;
        write_unaligned(src_ptr, opcode);

        if N == 5 {
            // Relative 32-bit displacement
            let displacement = (a_dst as isize - (a_src as isize + 5)) as i32;
            let displacement_ptr = src_ptr.add(1) as *mut i32;
            write_unaligned(displacement_ptr, displacement);
        } else if N == 6 {
            // Absolute 64-bit address
            let displacement_ptr = src_ptr.add(1) as *mut u8;
            write_unaligned(displacement_ptr, 0); // ModRM byte for indirect addressing
            let address_ptr = src_ptr.add(2) as *mut usize;
            write_unaligned(address_ptr, a_dst);
        } else {
            panic!("Invalid branch size");
        }

        a_src + N // Return the address after the written instruction
    }

    pub const unsafe fn write_call<const N: usize>(a_src: usize, a_dst: usize) -> usize {
        let data: u8 = match N {
            5 => 0xE8, // CALL rel32
            6 => 0x15, // CALL r/m64
            _ => panic!("invalid call size"),
        };

        Self::write_branch_with_data::<N>(a_src, a_dst, data)
    }
}
