use rand::distributions::Distribution;
use rand::prelude::StdRng;
use rand::rngs::ThreadRng;
use rand::SeedableRng;
use zipf::ZipfDistribution;
use common::config::PageId;

pub(crate) trait GetThreadPageIdGetter {
    fn get(&mut self) -> PageId;
}

pub(crate) struct RandomPageIdGetter {
    rng: ThreadRng,
    dist: ZipfDistribution,
}

impl RandomPageIdGetter {
    fn new(min_page_id: PageId, max_page_id: PageId) -> Self {
        let rng = rand::thread_rng();
        let dist = ZipfDistribution::new((max_page_id - min_page_id) as usize, 0.8).unwrap();

        Self {
            dist,
            rng,
        }
    }
}

impl GetThreadPageIdGetter for RandomPageIdGetter {
    fn get(&mut self) -> PageId {
        self.dist.sample(&mut self.rng) as PageId
    }
}

pub(crate) struct SeedableRandomPageIdGetter {
    rng: StdRng,
    dist: ZipfDistribution,
}

impl SeedableRandomPageIdGetter {
    fn new(min_page_id: PageId, max_page_id: PageId, seed: <StdRng as SeedableRng>::Seed) -> Self {
        let rng = StdRng::from_seed(seed);
        let dist = ZipfDistribution::new((max_page_id - min_page_id) as usize, 0.8).unwrap();

        Self {
            dist,
            rng,
        }
    }
}

impl GetThreadPageIdGetter for SeedableRandomPageIdGetter {
    fn get(&mut self) -> PageId {
        self.dist.sample(&mut self.rng) as PageId
    }
}


pub(crate) struct SequentialPageIdGetter {
    current_page_id: PageId,
    min_page_id: PageId,
    max_page_id: PageId,
}

impl SequentialPageIdGetter {
    fn new(min_page_id: PageId, max_page_id: PageId) -> Self {
        Self {
            current_page_id: min_page_id,
            min_page_id,
            max_page_id,
        }
    }
}

impl GetThreadPageIdGetter for SequentialPageIdGetter {

    fn get(&mut self) -> PageId {
        let page_id = self.current_page_id;

        self.current_page_id += 1;
        if self.current_page_id > self.max_page_id {
            self.current_page_id = self.min_page_id;
        }

        page_id
    }
}



pub(crate) struct ReversedSequentialPageIdGetter {
    current_page_id: PageId,
    min_page_id: PageId,
    max_page_id: PageId,
}

impl ReversedSequentialPageIdGetter {
    fn new(min_page_id: PageId, max_page_id: PageId) -> Self {
        Self {
            current_page_id: max_page_id,
            min_page_id,
            max_page_id,
        }
    }
}

impl GetThreadPageIdGetter for ReversedSequentialPageIdGetter {
    fn get(&mut self) -> PageId {
        let page_id = self.current_page_id;

        self.current_page_id = self.current_page_id.wrapping_sub(1);

        if self.current_page_id == page_id || self.current_page_id < self.min_page_id {
            self.current_page_id = self.max_page_id;
        }

        page_id
    }
}

#[derive(Debug, Clone)]
pub enum GetThreadPageId {
    #[allow(unused)]
    Random,
    #[allow(unused)]
    SeedableRandom(<StdRng as SeedableRng>::Seed),
    #[allow(unused)]
    Sequential,
    #[allow(unused)]
    SequentialReversed
}

impl Default for GetThreadPageId {
    fn default() -> Self {
        GetThreadPageId::Random
    }
}

impl GetThreadPageId {
    pub(crate) fn create_getter(&self, min_page_id: PageId, max_page_id: PageId) -> Box<dyn GetThreadPageIdGetter> {
        match self {
            GetThreadPageId::Random => Box::new(RandomPageIdGetter::new(min_page_id, max_page_id)),
            GetThreadPageId::SeedableRandom(seed) => Box::new(SeedableRandomPageIdGetter::new(min_page_id, max_page_id, *seed)),
            GetThreadPageId::Sequential => Box::new(SequentialPageIdGetter::new(min_page_id, max_page_id)),
            GetThreadPageId::SequentialReversed => Box::new(ReversedSequentialPageIdGetter::new(min_page_id, max_page_id)),
        }
    }
}
