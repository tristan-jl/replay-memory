use pyo3::{
    exceptions::PyIndexError, pyclass, pymethods, pymodule, types::PyModule, PyObject, PyResult,
    Python,
};

use rand::seq::SliceRandom;

#[pyclass]
#[derive(Debug)]
pub struct ReplayMemory {
    index: usize,
    data: Vec<PyObject>,
}

#[pymethods]
impl ReplayMemory {
    #[new]
    pub fn new(capacity: usize) -> Self {
        Self {
            index: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: usize) -> PyResult<&PyObject> {
        self.data
            .get(index)
            .ok_or_else(|| PyIndexError::new_err("index is out of range"))
    }

    pub fn push(&mut self, item: PyObject) {
        if !self.is_full() {
            self.data.push(item)
        } else {
            self.data[self.index] = item;
        }
        self.index += 1;
        self.index %= self.data.capacity();
    }

    pub fn push_items(&mut self, items: Vec<PyObject>) {
        for item in items {
            self.push(item);
        }
    }

    pub fn sample(&self, sample_size: usize) -> Vec<PyObject> {
        self.data
            .choose_multiple(&mut rand::thread_rng(), sample_size)
            .cloned()
            .collect()
    }

    #[getter]
    pub fn is_full(&self) -> bool {
        self.data.len() == self.data.capacity()
    }

    #[getter]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn __getitem__(&self, idx: usize) -> PyResult<&PyObject> {
        self.get(idx)
    }

    pub fn __len__(&self) -> PyResult<usize> {
        Ok(self.data.len())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        let mut repr = String::from("ReplayMemory([");
        let mut items = self.data.iter().peekable();

        while let Some(item) = items.next() {
            repr.push_str(&item.to_string());
            if items.peek().is_some() {
                repr.push_str(", ");
            }
        }

        repr.push_str("])");
        Ok(repr)
    }
}

#[pymodule]
fn replay_memory(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ReplayMemory>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use pyo3::ToPyObject;

    use super::*;

    #[test]
    fn overwrites_oldest() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rb = ReplayMemory::new(5);
            (0..6).for_each(|i| {
                rb.push(i.to_object(py));
            });
            assert_eq!(
                rb.data,
                vec![5, 1, 2, 3, 4]
                    .iter()
                    .map(|i| i.to_object(py))
                    .collect::<Vec<PyObject>>()
            )
        });
    }

    #[test]
    fn capacity_unchanged() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rb = ReplayMemory::new(5);
            (0..500).for_each(|i| {
                rb.push(i.to_object(py));
            });
            assert_eq!(rb.data.capacity(), 5);
            assert_eq!(rb.data.len(), 5);
        });
    }

    #[test]
    fn samples() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rb = ReplayMemory::new(5);
            (0..5).for_each(|i| {
                rb.push(i.to_object(py));
            });
            assert_eq!(rb.sample(3).len(), 3);
        });
    }
}
