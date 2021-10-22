use tile_editor::graphics::{Material, ShaderFeatures, ShaderModuleProvider};

#[test]
fn shader_provider_1() {
    let source = include_str!("test_file.txt");
    let features = ShaderFeatures {
        material_flags: Material::DIFFUSE_BIT | Material::NORMAL_BIT
    };
    let expected = "diffuse is here!\nnormal is here!\ncontent\n";
    let actual = ShaderModuleProvider::preprocess_source(source, &features);
    assert_eq!(expected, actual);
}

#[test]
fn shader_provider_2() {
    let source = include_str!("test_file.txt");
    let features = ShaderFeatures {
        material_flags: Material::DIFFUSE_BIT
    };
    let expected = "diffuse is here!\ncontent\n";
    let actual = ShaderModuleProvider::preprocess_source(source, &features);
    assert_eq!(expected, actual);
}

#[test]
fn shader_provider_3() {
    let source = include_str!("test_file.txt");
    let features = ShaderFeatures {
        material_flags: Material::NORMAL_BIT
    };
    let expected = "normal is here!\ncontent\n";
    let actual = ShaderModuleProvider::preprocess_source(source, &features);
    assert_eq!(expected, actual);
}

#[test]
fn shader_provider_4() {
    let source = include_str!("test_file.txt");
    let features = ShaderFeatures { material_flags: 0 };
    let expected = "content\n";
    let actual = ShaderModuleProvider::preprocess_source(source, &features);
    assert_eq!(expected, actual);
}