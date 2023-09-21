type G1Affine = <Bls12<ark_bls12_381::Parameters> as ark_ec::PairingEngine>::G1Affine;
type G2Affine = <Bls12<ark_bls12_381::Parameters> as ark_ec::PairingEngine>::G2Affine;
type G2Prepared = <Bls12<ark_bls12_381::Parameters> as ark_ec::PairingEngine>::G2Prepared;
use ark_ec::bls12::Bls12;
use ark_ff::QuadExtField;
use ark_serialize::Write;
use num_traits::ToBytes;
use std::collections::BTreeMap;
use std::io::{Read, Result};

pub fn store_params(pp: ark_poly_commit::kzg10::UniversalParams<Bls12<ark_bls12_381::Parameters>>, name: &str) {
    // MISC FILE - contains size of former vectors
    let file = std::fs::File::create(name.to_string() + ".misc").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    // POWERS OF G
    writer.write_all((pp.powers_of_g.len()).to_le_bytes().as_ref()).unwrap();
    println!("LENGTH POWERS OF G: {}", &pp.powers_of_g.len());
    store_slice(&pp.powers_of_g, &(name.to_string() + ".powers_of_g"));

    // POWERS OF GAMMA G
    writer.write_all((&pp.powers_of_gamma_g.len()).to_le_bytes().as_ref()).unwrap();
    println!("LENGTH POWERS OF GAMMA G: {}", &pp.powers_of_gamma_g.len());
    let vec: Vec<(usize, G1Affine)> = pp.powers_of_gamma_g.iter().map(|(k, v)| (*k, *v)).collect();
    store_slice(&vec, &(name.to_string() + ".powers_of_gamma_g"));

    // H
    store(&pp.h, &(name.to_string() + ".h"));

    // BETA H
    store(&pp.beta_h, &(name.to_string() + ".beta_h"));

    // NEG POWERS OF H
    writer.write_all((&pp.neg_powers_of_h.len()).to_le_bytes().as_ref()).unwrap();
    println!("LENGTH NEG POWERS OF H: {}", &pp.neg_powers_of_h.len());
    let vec: Vec<(usize, G2Affine)> = pp.neg_powers_of_h.iter().map(|(k, v)| (*k, *v)).collect();
    store_slice(&vec, &(name.to_string() + ".neg_powers_of_h"));

    // PREPARED H
    writer.write_all((&pp.prepared_h.ell_coeffs.len()).to_le_bytes().as_ref()).unwrap();
    let b_byte = if pp.prepared_h.infinity { 1u8 } else { 0u8 };
    writer.write_all(&[b_byte]).unwrap();
    println!("PREPARED H INFINITY: {}", &pp.prepared_h.infinity);
    println!("LENGTH ELL COEFS PREPARED H INFINITY: {}", &pp.prepared_h.ell_coeffs.len());
    let vec = pp.prepared_h.ell_coeffs.clone();
    store_slice(&vec, &(name.to_string() + ".prepared_h"));

    // PREPARED BETA H
    writer
        .write_all((&pp.prepared_beta_h.ell_coeffs.len()).to_le_bytes().as_ref())
        .unwrap();
    let b_byte = if pp.prepared_beta_h.infinity { 1u8 } else { 0u8 };
    writer.write_all(&[b_byte]).unwrap();
    println!("PREPARED BETA H INFINITY: {}", &pp.prepared_beta_h.infinity);
    println!(
        "LENGTH ELL COEFS PREPARED BETA H INFINITY: {}",
        &pp.prepared_beta_h.ell_coeffs.len()
    );
    let vec = pp.prepared_beta_h.ell_coeffs.clone();
    store_slice(&vec, &(name.to_string() + ".prepared_beta_h"));
}

pub fn load_params(name: &str) -> ark_poly_commit::kzg10::UniversalParams<Bls12<ark_bls12_381::Parameters>> {
    // load size of vectors from misc file
    let mut file = std::fs::File::open(name.to_string() + ".misc").unwrap();
    let len_powers_of_g = read_usize(&mut file).unwrap();
    let len_powers_of_gamma_g = read_usize(&mut file).unwrap();
    let len_neg_powers_oh_h = read_usize(&mut file).unwrap();
    let len_prepared_h = read_usize(&mut file).unwrap();
    let inf_prepared_h = read_bool(&mut file).unwrap();
    let len_prepared_beta_h = read_usize(&mut file).unwrap();
    let inf_prepared_beta_h = read_bool(&mut file).unwrap();

    // POWERS OF G
    let mut powers_of_g: Vec<G1Affine> = vec![G1Affine::default(); len_powers_of_g];
    load_slice(&mut powers_of_g, &(name.to_string() + ".powers_of_g"));

    // POWERS OF GAMMA G
    let mut contents: Vec<(usize, G1Affine)> = vec![(0, G1Affine::default()); len_powers_of_gamma_g];
    let mut powers_of_gamma_g: std::collections::BTreeMap<usize, G1Affine> = BTreeMap::new();
    load_slice(&mut contents, &(name.to_string() + ".powers_of_gamma_g"));
    contents.into_iter().enumerate().for_each(|(i, (_, val))| {
        powers_of_gamma_g.insert(i, val);
    });

    // H
    let mut h = G2Affine::default();
    load(&mut h, &(name.to_string() + ".h"));

    // BETA H
    let mut beta_h = G2Affine::default();
    load(&mut beta_h, &(name.to_string() + ".beta_h"));

    // NEG POWERS OF H
    let mut contents: Vec<(usize, G2Affine)> = vec![(0, G2Affine::default()); len_neg_powers_oh_h];
    let mut neg_powers_of_h = std::collections::BTreeMap::<usize, G2Affine>::new();
    load_slice(&mut contents, &(name.to_string() + ".neg_powers_of_h"));
    contents.into_iter().enumerate().for_each(|(i, (_, val))| {
        neg_powers_of_h.insert(i, val);
    });

    // PREPARED H
    let mut ell_coefs = vec![(QuadExtField::default(), QuadExtField::default(), QuadExtField::default()); len_prepared_h];
    load_slice(&mut ell_coefs, &(name.to_string() + ".prepared_h"));
    let prepared_h = G2Prepared {
        ell_coeffs: ell_coefs,
        infinity: inf_prepared_h,
    };

    // PREPARED BETA H
    let mut ell_coefs = vec![(QuadExtField::default(), QuadExtField::default(), QuadExtField::default()); len_prepared_beta_h];
    load_slice(&mut ell_coefs, &(name.to_string() + ".prepared_beta_h"));
    let prepared_beta_h = G2Prepared {
        ell_coeffs: ell_coefs,
        infinity: inf_prepared_beta_h,
    };

    // Create and return the UniversalParams struct
    ark_poly_commit::kzg10::UniversalParams {
        powers_of_g: powers_of_g,
        powers_of_gamma_g: powers_of_gamma_g,
        h: h,
        beta_h: beta_h,
        neg_powers_of_h: neg_powers_of_h,
        prepared_h: prepared_h,
        prepared_beta_h: prepared_beta_h,
    }
}

fn read_usize(file: &mut std::fs::File) -> Result<usize> {
    let mut buf = [0u8; 8];
    file.read_exact(&mut buf)?;
    Ok(usize::from_le_bytes(buf))
}

fn read_bool(file: &mut std::fs::File) -> Result<bool> {
    let mut buf = [0u8; 1];
    file.read_exact(&mut buf)?;
    Ok(buf[0] == 1u8)
}

fn load<T: Sized>(data: &mut T, name: &str) {
    use std::io::Read as _;
    let size = std::mem::size_of::<T>();
    println!("size of variable: {}B", std::mem::size_of_val(data));
    std::fs::File::open(name)
        .unwrap_or_else(|_| panic!("no such file {}", name))
        .read_exact(unsafe { std::slice::from_raw_parts_mut(data as *mut T as *mut u8, size) })
        .unwrap();
    println!("load {}B from {}\n", size, name);
}

fn store<T: Sized>(data: &T, name: &str) {
    use std::io::Write as _;
    let size = std::mem::size_of::<T>();
    std::fs::File::create(name)
        .unwrap()
        .write_all(unsafe { std::slice::from_raw_parts(data as *const T as *const u8, size) })
        .unwrap();
    println!("store {}B to {}", size, name);
}

fn store_slice<T: Sized>(slice: &[T], name: &str) {
    use std::io::Write as _;
    let slice_data_size = std::mem::size_of::<T>() * slice.len();
    std::fs::File::create(name)
        .unwrap()
        .write_all(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice_data_size) })
        .unwrap();
    println!("store {}B to {}", slice_data_size, name);
}

fn load_slice<T: Sized>(slice: &mut [T], name: &str) {
    use std::io::Read as _;
    let slice_data_size = std::mem::size_of::<T>() * slice.len();
    std::fs::File::open(name)
        .unwrap()
        .read_exact(unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut u8, slice_data_size) })
        .unwrap();
    println!("load {}B from {}", slice_data_size, name);
}
