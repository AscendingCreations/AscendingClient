use bytey::{ByteBuffer, ByteBufferRead, ByteBufferWrite};
use mmap_bytey::{MByteBuffer, MByteBufferRead, MByteBufferWrite};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyDuration(pub chrono::Duration);

impl MyDuration {
    pub fn milliseconds(mills: i64) -> MyDuration {
        MyDuration(
            chrono::Duration::try_milliseconds(mills).unwrap_or_default(),
        )
    }

    pub fn as_std(&self) -> std::time::Duration {
        if let Ok(dur) = self.0.to_std() {
            dur
        } else {
            std::time::Duration::from_millis(0)
        }
    }
}

impl From<chrono::Duration> for MyDuration {
    fn from(duration: chrono::Duration) -> MyDuration {
        MyDuration(duration)
    }
}

impl AsRef<chrono::Duration> for MyDuration {
    fn as_ref(&self) -> &chrono::Duration {
        &self.0
    }
}

impl std::ops::Deref for MyDuration {
    type Target = chrono::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for MyDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.num_milliseconds().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MyDuration {
    fn deserialize<D>(deserializer: D) -> Result<MyDuration, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(i64::deserialize(deserializer)?)
                .unwrap_or_default(),
        ))
    }
}

impl ByteBufferRead for MyDuration {
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> bytey::Result<Self> {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(buffer.read::<i64>()?)
                .unwrap_or_default(),
        ))
    }

    fn read_from_bytey_buffer_le(
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<Self> {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(buffer.read_le::<i64>()?)
                .unwrap_or_default(),
        ))
    }

    fn read_from_bytey_buffer_be(
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<Self> {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(buffer.read_be::<i64>()?)
                .unwrap_or_default(),
        ))
    }
}

impl ByteBufferWrite for &MyDuration {
    fn write_to_bytey_buffer(
        &self,
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<()> {
        buffer.write(self.num_milliseconds())?;
        Ok(())
    }
    fn write_to_bytey_buffer_le(
        &self,
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<()> {
        buffer.write_le(self.num_milliseconds())?;
        Ok(())
    }
    fn write_to_bytey_buffer_be(
        &self,
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<()> {
        buffer.write_be(self.num_milliseconds())?;
        Ok(())
    }
}

impl MByteBufferRead for MyDuration {
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> mmap_bytey::Result<Self> {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(buffer.read::<i64>()?)
                .unwrap_or_default(),
        ))
    }

    fn read_from_mbuffer_le(
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<Self> {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(buffer.read_le::<i64>()?)
                .unwrap_or_default(),
        ))
    }

    fn read_from_mbuffer_be(
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<Self> {
        Ok(MyDuration(
            chrono::Duration::try_milliseconds(buffer.read_be::<i64>()?)
                .unwrap_or_default(),
        ))
    }
}

impl MByteBufferWrite for &MyDuration {
    fn write_to_mbuffer(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write(self.num_milliseconds())?;
        Ok(())
    }
    fn write_to_mbuffer_le(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write_le(self.num_milliseconds())?;
        Ok(())
    }
    fn write_to_mbuffer_be(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write_be(self.num_milliseconds())?;
        Ok(())
    }
}
