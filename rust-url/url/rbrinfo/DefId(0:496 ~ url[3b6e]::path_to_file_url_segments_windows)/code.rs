fn path_to_file_url_segments_windows(
    path: &Path,
    serialization: &mut String,
) -> Result<(u32, HostInternal), ()> {
    use crate::parser::PATH_SEGMENT;
    use percent_encoding::percent_encode;
    use std::path::{Component, Prefix};
    if !path.is_absolute() {
        return Err(());
    }
    let mut components = path.components();

    let host_start = serialization.len() + 1;
    let host_end;
    let host_internal;

    match components.next() {
        Some(Component::Prefix(ref p)) => match p.kind() {
            Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                host_end = to_u32(serialization.len()).unwrap();
                host_internal = HostInternal::None;
                serialization.push('/');
                serialization.push(letter as char);
                serialization.push(':');
            }
            Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => {
                let host = Host::parse(server.to_str().ok_or(())?).map_err(|_| ())?;
                write!(serialization, "{}", host).unwrap();
                host_end = to_u32(serialization.len()).unwrap();
                host_internal = host.into();
                serialization.push('/');
                let share = share.to_str().ok_or(())?;
                serialization.extend(percent_encode(share.as_bytes(), PATH_SEGMENT));
            }
            _ => return Err(()),
        },
        _ => return Err(()),
    }

    let mut path_only_has_prefix = true;
    for component in components {
        if component == Component::RootDir {
            continue;
        }

        path_only_has_prefix = false;
        // FIXME: somehow work with non-unicode?
        let component = component.as_os_str().to_str().ok_or(())?;

        serialization.push('/');
        serialization.extend(percent_encode(component.as_bytes(), PATH_SEGMENT));
    }

    // A windows drive letter must end with a slash.
    if serialization.len() > host_start
        && parser::is_windows_drive_letter(&serialization[host_start..])
        && path_only_has_prefix
    {
        serialization.push('/');
    }

    Ok((host_end, host_internal))
}