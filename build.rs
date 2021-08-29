

fn main() {
    windows::build!(
        Windows::Win32::System::Com::CoInitialize,
        Windows::Win32::System::PropertiesSystem::SHGetPropertyStoreFromParsingName,
        Windows::Win32::System::PropertiesSystem::IPropertyStore,
        Windows::Win32::System::PropertiesSystem::GETPROPERTYSTOREFLAGS,
        Windows::Win32::Storage::StructuredStorage::PROPVARIANT,
        Windows::Win32::System::PropertiesSystem::PROPERTYKEY,
        Windows::Win32::System::PropertiesSystem::InitPropVariantFromStringAsVector,
        Windows::Win32::System::PropertiesSystem::InitPropVariantFromUInt32Vector,
        Windows::Win32::System::PropertiesSystem::PSGetPropertyKeyFromName,
        Windows::Win32::System::PropertiesSystem::InitPropVariantFromPropVariantVectorElem,
        Windows::Win32::Foundation::PWSTR,
    );
}
