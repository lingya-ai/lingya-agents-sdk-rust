//! 增量 SSE frame 解码。

/// 可跨任意网络分片解析 SSE `data` 字段的状态机。
#[derive(Debug, Default)]
pub struct SseDecoder {
    pending: Vec<u8>,
    data_lines: Vec<String>,
}

impl SseDecoder {
    /// 输入一个任意大小的网络分片并返回其中已完成的事件数据。
    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<String>, std::str::Utf8Error> {
        self.pending.extend_from_slice(chunk);
        let mut completed = Vec::new();
        while let Some(index) = self.pending.iter().position(|byte| *byte == b'\n') {
            let mut line = self.pending.drain(..=index).collect::<Vec<_>>();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            self.consume_line(std::str::from_utf8(&line)?, &mut completed);
        }
        Ok(completed)
    }

    /// 在正常 EOF 时提交最后一个未以空行结尾的事件。
    pub fn finish(mut self) -> Result<Vec<String>, std::str::Utf8Error> {
        let mut completed = Vec::new();
        if !self.pending.is_empty() {
            let pending = std::mem::take(&mut self.pending);
            let line = std::str::from_utf8(&pending)?;
            self.consume_line(line.trim_end_matches('\r'), &mut completed);
        }
        self.dispatch(&mut completed);
        Ok(completed)
    }

    fn consume_line(&mut self, line: &str, completed: &mut Vec<String>) {
        if line.is_empty() {
            self.dispatch(completed);
        } else if let Some(data) = line.strip_prefix("data:") {
            self.data_lines
                .push(data.strip_prefix(' ').unwrap_or(data).to_owned());
        }
    }

    fn dispatch(&mut self, completed: &mut Vec<String>) {
        if !self.data_lines.is_empty() {
            completed.push(self.data_lines.join("\n"));
            self.data_lines.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SseDecoder;

    #[test]
    fn decodes_chunks_multiline_data_and_heartbeats() {
        let mut decoder = SseDecoder::default();
        assert!(decoder.push(b": ping\r\nda").unwrap().is_empty());
        assert_eq!(
            decoder.push(b"ta: {\"a\":\r\ndata: 1}\r\n\r\n").unwrap(),
            vec!["{\"a\":\n1}"]
        );
        assert!(decoder.finish().unwrap().is_empty());
    }
}
