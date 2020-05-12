import pickle
import pytest

from pyrpds import pset, s


def test_set():
    set_0 = pset()
    assert len(set_0) == 0
    assert hash(set_0) == -4800647303603446203

    set_1 = set_0.add(0)
    assert 0 in set_1
    assert len(set_1) == 1
    assert hash(set_1) == -8559946577813192710

    set_2 = set_1.add("1")
    assert 0 in set_2
    assert "1" in set_2
    assert len(set_2) == 2

    set_3 = set_2.add(2)
    assert 0 in set_3
    assert "1" in set_3
    assert 2 in set_3
    assert len(set_3) == 3

    set_4 = set_3.remove(2)
    assert 0 in set_2
    assert "1" in set_2
    assert len(set_4) == 2

    assert set_0 != set_1
    assert set_1 != set_2
    assert set_2 != set_3
    assert set_3 != set_4
    assert set_2 == set_4
    assert hash(set_2) == hash(set_4)


def test_pset_constructor():
    container = pset()
    assert len(container) == 0

    container = pset((0, "1", 2))
    assert len(container) == 3

    container = pset([0, "1", 2])
    assert len(container) == 3

    container = pset(range(5))
    assert len(container) == 5

    container = pset(map(lambda x: x, range(10)))
    assert len(container) == 10


def test_s_constructor():
    container = s()
    assert len(container) == 0

    container = s(0, "1", 2, 6, 10)
    assert len(container) == 5


def test_iter():
    container = pset()
    for element in range(100):
        container.add(element)
    for index, element in enumerate(sorted(container)):
        assert index == element


r"""
The rest of the test suite was copied from
https://github.com/tobgu/pyrsistent/blob/master/tests/set_test.py
"""


def test_literalish_works():
    assert s(1, 2) == pset([1, 2])


def test_supports_hash():
    # assert hash(s(1, 2)) == hash(s(1, 2))
    # pyrpds.pvector is order-agnostic
    assert hash(s(1)) == hash(s(1))


def test_empty_truthiness():
    assert s(1)
    assert not s()


def test_contains_elements_that_it_was_initialized_with():
    initial = [1, 2, 3]
    s = pset(initial)

    assert set(s) == set(initial)
    assert len(s) == len(set(initial))


def test_is_immutable():
    s1 = pset([1])
    s2 = s1.add(2)

    assert s1 == pset([1])
    assert s2 == pset([1, 2])

    s3 = s2.remove(1)
    assert s2 == pset([1, 2])
    assert s3 == pset([2])


def test_remove_when_not_present():
    s1 = s(1, 2, 3)
    with pytest.raises(KeyError):
        s1.remove(4)


def test_discard():
    s1 = s(1, 2, 3)
    assert s1.discard(3) == s(1, 2)
    assert s1.discard(4) == s1


def test_is_iterable():
    assert sum(pset([1, 2, 3])) == 6


def test_contains():
    s = pset([1, 2, 3])

    assert 2 in s
    assert 4 not in s


@pytest.mark.skip("Set operations are not implemented")
def test_supports_set_operations():
    s1 = pset([1, 2, 3])
    s2 = pset([3, 4, 5])

    assert s1 | s2 == s(1, 2, 3, 4, 5)
    assert s1.union(s2) == s1 | s2

    assert s1 & s2 == s(3)
    assert s1.intersection(s2) == s1 & s2

    assert s1 - s2 == s(1, 2)
    assert s1.difference(s2) == s1 - s2

    assert s1 ^ s2 == s(1, 2, 4, 5)
    assert s1.symmetric_difference(s2) == s1 ^ s2


@pytest.mark.skip("Set comparisons are not implemented")
def test_supports_set_comparisons():
    s1 = s(1, 2, 3)
    s3 = s(1, 2)
    s4 = s(1, 2, 3)

    assert s(1, 2, 3, 3, 5) == s(1, 2, 3, 5)
    assert s1 != s3

    assert s3 < s1
    assert s3 <= s1
    assert s3 <= s4

    assert s1 > s3
    assert s1 >= s3
    assert s4 >= s3


def test_str():
    rep = str(pset([1, 2, 3]))
    assert eval(rep) == eval("pset([1, 2, 3])")


def test_is_disjoint():
    s1 = pset([1, 2, 3])
    s2 = pset([3, 4, 5])
    s3 = pset([4, 5])

    assert not s1.isdisjoint(s2)
    assert s1.isdisjoint(s3)


@pytest.mark.skip("evolver is not currently supported")
def test_evolver_simple_add():
    x = s(1, 2, 3)
    e = x.evolver()
    assert not e.is_dirty()

    e.add(4)
    assert e.is_dirty()

    x2 = e.persistent()
    assert not e.is_dirty()
    assert x2 == s(1, 2, 3, 4)
    assert x == s(1, 2, 3)


@pytest.mark.skip("evolver is not currently supported")
def test_evolver_simple_remove():
    x = s(1, 2, 3)
    e = x.evolver()
    e.remove(2)

    x2 = e.persistent()
    assert x2 == s(1, 3)
    assert x == s(1, 2, 3)


@pytest.mark.skip("evolver is not currently supported")
def test_evolver_no_update_produces_same_pset():
    x = s(1, 2, 3)
    e = x.evolver()
    assert e.persistent() is x


@pytest.mark.skip("evolver is not currently supported")
def test_evolver_len():
    x = s(1, 2, 3)
    e = x.evolver()
    assert len(e) == 3


@pytest.mark.skip("copy is not currently supported")
def test_copy_returns_reference_to_self():
    s1 = s(10)
    assert s1.copy() is s1


@pytest.mark.skip("pickling is not currently supported")
def test_pickling_empty_set():
    assert pickle.loads(pickle.dumps(s(), -1)) == s()


@pytest.mark.skip("pickling is not currently supported")
def test_pickling_non_empty_map():
    assert pickle.loads(pickle.dumps(s(1, 2), -1)) == s(1, 2)


@pytest.mark.skip("weakref is not currently supported")
def test_supports_weakref():
    import weakref

    weakref.ref(s(1))


def test_update():
    assert s(1, 2, 3).update([3, 4, 4, 5]) == s(1, 2, 3, 4, 5)


@pytest.mark.skip("Currently not supported")
def test_update_no_elements():
    s1 = s(1, 2)
    assert s1.update([]) is s1


def test_iterable():
    """
    PSets can be created from iterables even though they can't be len() hinted.
    """

    assert pset(iter("a")) == pset(iter("a"))
