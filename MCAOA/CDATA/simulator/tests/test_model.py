"""Tests for CDATA-v2 simulator."""

import numpy as np
from cdata_sim import CDATAModel, CDATAParams


def test_model_creation():
    model = CDATAModel(seed=42)
    assert model.params.mu_P > 0
    assert model.params.alpha_AurA_215 > 0
    assert model.params.sigma_N > 0


def test_single_tree():
    model = CDATAModel(seed=42)
    trees = model.simulate_tree(max_generations=40, n_cells=1)
    assert len(trees) == 1
    assert len(trees[0]) > 0


def test_multiple_trees():
    model = CDATAModel(seed=42)
    trees = model.simulate_tree(max_generations=40, n_cells=10)
    assert len(trees) == 10


def test_statistics():
    model = CDATAModel(seed=42)
    trees = model.simulate_tree(max_generations=40, n_cells=50)
    stats = model.compute_statistics(trees)
    assert "hayflick_median" in stats
    assert "hayflick_iqr" in stats
    assert stats["hayflick_median"] > 0


def test_p53_net():
    model = CDATAModel()
    assert model._p53_net(N_mat=1, M_f=1.0, D=0.0) >= 0.0
    assert model._p53_net(N_mat=3, M_f=1.0, D=0.0) > 0.0
    # Повреждения активируют p53 даже при N_mat=1
    assert model._p53_net(N_mat=1, M_f=1.0, D=3.0) > 0.0


def test_serialization():
    model = CDATAModel(seed=42)
    d = model.to_dict()
    model2 = CDATAModel.from_dict(d, seed=42)
    assert model2.params.mu_P == model.params.mu_P


def test_reproducibility():
    model1 = CDATAModel(seed=42)
    model2 = CDATAModel(seed=42)
    trees1 = model1.simulate_tree(max_generations=30, n_cells=5)
    trees2 = model2.simulate_tree(max_generations=30, n_cells=5)
    for t1, t2 in zip(trees1, trees2):
        for s1, s2 in zip(t1, t2):
            assert s1.generation == s2.generation
            assert abs(s1.D - s2.D) < 1e-10
