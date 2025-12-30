use bytey::{ByteBuffer, ByteBufferRead, ByteBufferWrite};
use mmap_bytey::{MByteBuffer, MByteBufferRead, MByteBufferWrite};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Add;

use coarsetime::{Duration, Instant};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MyInstant(pub Instant);

impl MyInstant {
    pub fn now() -> MyInstant {
        MyInstant(Instant::now())
    }

    pub fn recent() -> MyInstant {
        MyInstant(Instant::recent())
    }

    pub fn to_dur(self) -> i64 {
        let mut dur: i64 = 0;

        if let Ok(approx) =
            chrono::Duration::from_std(self.0.elapsed_since_recent().into())
            && approx
                > chrono::Duration::try_milliseconds(15).unwrap_or_default()
        {
            dur = approx.num_milliseconds();
        }

        dur
    }

    pub fn from_dur(dur: i64) -> MyInstant {
        let duration =
            chrono::Duration::try_milliseconds(dur).unwrap_or_default();
        let mut instant_now = Instant::now();

        if let Ok(dur) = duration.to_std() {
            let dur = Duration::from(dur);
            instant_now += dur;
        }

        MyInstant(instant_now)
    }
}

impl From<Instant> for MyInstant {
    fn from(instant: Instant) -> MyInstant {
        MyInstant(instant)
    }
}

impl AsRef<Instant> for MyInstant {
    fn as_ref(&self) -> &Instant {
        &self.0
    }
}

impl std::ops::Deref for MyInstant {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for MyInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_dur().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MyInstant {
    fn deserialize<D>(deserializer: D) -> Result<MyInstant, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MyInstant::from_dur(i64::deserialize(deserializer)?))
    }
}

impl ByteBufferRead for MyInstant {
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> bytey::Result<Self> {
        Ok(MyInstant::from_dur(buffer.read::<i64>()?))
    }

    fn read_from_bytey_buffer_le(
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<Self> {
        Ok(MyInstant::from_dur(buffer.read_le::<i64>()?))
    }

    fn read_from_bytey_buffer_be(
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<Self> {
        Ok(MyInstant::from_dur(buffer.read_be::<i64>()?))
    }
}

impl ByteBufferWrite for &MyInstant {
    fn write_to_bytey_buffer(
        &self,
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<()> {
        buffer.write(self.to_dur())?;
        Ok(())
    }
    fn write_to_bytey_buffer_le(
        &self,
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<()> {
        buffer.write_le(self.to_dur())?;
        Ok(())
    }
    fn write_to_bytey_buffer_be(
        &self,
        buffer: &mut ByteBuffer,
    ) -> bytey::Result<()> {
        buffer.write_be(self.to_dur())?;
        Ok(())
    }
}

impl MByteBufferRead for MyInstant {
    fn read_from_mbuffer(buffer: &mut MByteBuffer) -> mmap_bytey::Result<Self> {
        Ok(MyInstant::from_dur(buffer.read::<i64>()?))
    }

    fn read_from_mbuffer_le(
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<Self> {
        Ok(MyInstant::from_dur(buffer.read_le::<i64>()?))
    }

    fn read_from_mbuffer_be(
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<Self> {
        Ok(MyInstant::from_dur(buffer.read_be::<i64>()?))
    }
}

impl MByteBufferWrite for &MyInstant {
    fn write_to_mbuffer(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write(self.to_dur())?;
        Ok(())
    }
    fn write_to_mbuffer_le(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write_le(self.to_dur())?;
        Ok(())
    }
    fn write_to_mbuffer_be(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write_be(self.to_dur())?;
        Ok(())
    }
}

impl MByteBufferWrite for MyInstant {
    fn write_to_mbuffer(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write(self.to_dur())?;
        Ok(())
    }
    fn write_to_mbuffer_le(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write_le(self.to_dur())?;
        Ok(())
    }
    fn write_to_mbuffer_be(
        &self,
        buffer: &mut MByteBuffer,
    ) -> mmap_bytey::Result<()> {
        buffer.write_be(self.to_dur())?;
        Ok(())
    }
}

impl Add<chrono::Duration> for MyInstant {
    type Output = MyInstant;

    fn add(self, other: chrono::Duration) -> MyInstant {
        if let Ok(dur) = other.to_std() {
            let dur = Duration::from(dur);
            MyInstant(self.0 + dur)
        } else {
            MyInstant(self.0)
        }
    }
}

impl Add<std::time::Duration> for MyInstant {
    type Output = MyInstant;

    fn add(self, other: std::time::Duration) -> MyInstant {
        let dur = Duration::from(other);
        MyInstant(self.0 + dur)
    }
}
