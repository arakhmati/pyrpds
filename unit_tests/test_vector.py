from pyrpds import pvector, v


def test_vector():
    vector_0 = pvector()

    assert len(vector_0) == 0
    assert hash(vector_0) == -4800647303603446203

    try:
        vector_0.get(-1)
    except OverflowError:
        ...

    try:
        vector_0.get(0)
    except IndexError:
        ...

    try:
        vector_0.set(-1, 0)
    except OverflowError:
        ...

    try:
        vector_0.set(0, 0)
    except IndexError:
        ...

    vector_1 = vector_0.append(0)
    assert vector_1.get(0) == 0
    assert vector_1[0] == 0
    assert 0 in vector_1
    assert len(vector_1) == 1
    assert hash(vector_1) == -8559946577813192710

    vector_2 = vector_1.append("1")
    assert vector_2.get(0) == 0
    assert vector_2[0] == 0
    assert vector_2.get(1) == "1"
    assert vector_2[1] == "1"
    assert len(vector_2) == 2

    vector_3 = vector_2.append(2)
    assert vector_2.get(0) == 0
    assert vector_3.get(1) == "1"
    assert vector_3.get(2) == 2
    assert len(vector_3) == 3

    vector_4 = vector_3.set(0, "zero")
    assert vector_4.get(0) == "zero"
    assert vector_4.get(1) == "1"
    assert vector_4.get(2) == 2
    assert len(vector_4) == 3

    vector_5 = vector_4.extend([3, "4"])
    assert vector_5.get(0) == "zero"
    assert vector_5.get(1) == "1"
    assert vector_5.get(2) == 2
    assert vector_5.get(3) == 3
    assert vector_5.get(4) == "4"
    assert len(vector_5) == 5

    vector_6 = vector_1.append("1")
    assert len(vector_6) == 2

    assert vector_0 != vector_1
    assert vector_1 != vector_2
    assert vector_2 != vector_3
    assert vector_2 == vector_6
    assert hash(vector_2) == hash(vector_6)


def test_pvector_constuctor():
    container = pvector()
    assert len(container) == 0

    container = pvector((0, "1", 2))
    assert len(container) == 3

    container = pvector([0, "1", 2])
    assert len(container) == 3

    container = pvector(range(5))
    assert len(container) == 5

    container = pvector(map(lambda x: x, range(10)))
    assert len(container) == 10


def test_v_constuctor():
    container = v()
    assert len(container) == 0

    container = v(0, "1", 2, 6, 10)
    assert len(container) == 5


def test_iter():
    container = pvector()
    for element in range(100):
        container.append(element)
    for index, element in enumerate(sorted(container)):
        assert index == element
