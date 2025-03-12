fn set_port_internal(&mut self, port: Option<u16>) {
        match (self.port, port) {
            (None, None) => {}
            (Some(_), None) => {
                self.serialization
                    .drain(self.host_end as usize..self.path_start as usize);
                let offset = self.path_start - self.host_end;
                self.path_start = self.host_end;
                if let Some(ref mut index) = self.query_start {
                    *index -= offset
                }
                if let Some(ref mut index) = self.fragment_start {
                    *index -= offset
                }
            }
            (Some(old), Some(new)) if old == new => {}
            (_, Some(new)) => {
                let path_and_after = self.slice(self.path_start..).to_owned();
                self.serialization.truncate(self.host_end as usize);
                write!(&mut self.serialization, ":{}", new).unwrap();
                let old_path_start = self.path_start;
                let new_path_start = to_u32(self.serialization.len()).unwrap();
                self.path_start = new_path_start;
                let adjust = |index: &mut u32| {
                    *index -= old_path_start;
                    *index += new_path_start;
                };
                if let Some(ref mut index) = self.query_start {
                    adjust(index)
                }
                if let Some(ref mut index) = self.fragment_start {
                    adjust(index)
                }
                self.serialization.push_str(&path_and_after);
            }
        }
        self.port = port;
    }