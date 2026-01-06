# Actor Movie SQL Queries

## Basic Queries

### 1. Find the name and year of the movies
**Task:** Write a SQL query to find the name and year of the movies. Return movie title, movie release year.

```sql
SELECT mov_title, mov_year 
FROM movie;
```

**Output:**
| Movie Title                 | Year |
|----------------------------|------|
| Vertigo                    | 1958 |
| The Innocents              | 1961 |
| Lawrence of Arabia         | 1962 |
| The Deer Hunter            | 1978 |
| Amadeus                    | 1984 |
| Blade Runner               | 1982 |
| Eyes Wide Shut             | 1999 |
| The Usual Suspects         | 1995 |
| Chinatown                  | 1974 |
| Boogie Nights              | 1997 |
| Annie Hall                 | 1977 |
| Princess Mononoke          | 1997 |
| The Shawshank Redemption   | 1994 |
| American Beauty            | 1999 |
| Titanic                    | 1997 |
| Good Will Hunting          | 1997 |
| Deliverance                | 1972 |
| Trainspotting              | 1996 |
| The Prestige               | 2006 |
| Donnie Darko               | 2001 |
| Slumdog Millionaire        | 2008 |
| Aliens                     | 1986 |
| Beyond the Sea             | 2004 |
| Avatar                     | 2009 |
| Seven Samurai              | 1954 |
| Spirited Away              | 2001 |
| Back to the Future         | 1985 |
| Braveheart                 | 1995 |


---

### 2. Find when 'American Beauty' was released
**Task:** Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.

```sql
SELECT mov_year 
FROM movie 
WHERE mov_title = "American Beauty";
```

**Output:**
| mov_year |
|----------|
| 1999     |


---

### 3. Find movies released in 1999
**Task:** Write a SQL query to find the movie that was released in 1999. Return movie title.

```sql
SELECT mov_title 
FROM movie 
WHERE mov_year = YEAR(mov_dt_rel);
```

**Output:**
| mov_title               |
|-------------------------|
| Vertigo                 |
| Lawrence of Arabia      |
| Blade Runner            |
| The Usual Suspects      |
| Chinatown               |
| Annie Hall              |
| Trainspotting           |
| The Prestige            |
| Aliens                  |
| Beyond the Sea          |
| Avatar                  |
| Seven Samurai           |
| Back to the Future      |
| Braveheart              |


---

### 4. Find movies released before 1998
**Task:** Write a SQL query to find those movies, which were released before 1998. Return movie title.

```sql
SELECT mov_title 
FROM movie 
WHERE YEAR(mov_dt_rel) < 1998;
```

**Output:**
| mov_title                    |
|------------------------------|
| Vertigo                      |
| The Innocents                |
| Lawrence of Arabia           |
| The Deer Hunter              |
| Amadeus                      |
| Blade Runner                 |
| The Usual Suspects           |
| Chinatown                    |
| Annie Hall                   |
| The Shawshank Redemption     |
| Deliverance                  |
| Trainspotting                |
| Aliens                       |
| Seven Samurai                |
| Back to the Future           |
| Braveheart                   |


---

### 5. Find all reviewers and movies in a single list
**Task:** Write a SQL query to find the name of all reviewers and movies together in a single list.

```sql
SELECT rev_name 
FROM movie_reviewer 
UNION 
SELECT mov_title 
FROM movie;
```

**Output:**
|                          |
|--------------------------|
| Righty Sock              |
| Jack Malvern             |
| Flagrant Baronessa       |
| Alec Shaw                |
|                          |
| Victor Woeltjen          |
| Simon Wright             |
| Neal Wruck               |
| Paul Monks               |
| Mike Salvati             |
| Wesley S. Walker         |
| Sasha Goldshtein         |
| Josh Cates               |
| Krug Stillo              |
| Scott LeBrun             |
| Hannah Steele            |
| Vincent Cadena           |
| Brandt Sponseller        |
| Richard Adams            |
| Vertigo                  |
| The Innocents            |
| Lawrence of Arabia       |
| The Deer Hunter          |
| Amadeus                  |
| Blade Runner             |
| Eyes Wide Shut           |
| The Usual Suspects       |
| Chinatown                |
| Boogie Nights            |
| Annie Hall               |
| Princess Mononoke        |
| The Shawshank Redemption |
| American Beauty          |
| Titanic                  |
| Good Will Hunting        |
| Deliverance              |
| Trainspotting            |
| The Prestige             |
| Donnie Darko             |
| Slumdog Millionaire      |
| Aliens                   |
| Beyond the Sea           |
| Avatar                   |
| Seven Samurai            |
| Spirited Away            |
| Back to the Future       |
| Braveheart               |


---

### 6. Find reviewers who rated 7+ stars
**Task:** Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.

```sql
SELECT rev.rev_name 
FROM movie_reviewer AS rev, movie_rating AS rat 
WHERE rev.rev_id = rat.rev_id 
  AND rat.rev_stars >= 7;
```

**Output:**
| rev_name              |
|-----------------------|
| Righty Sock           |
| Jack Malvern          |
| Flagrant Baronessa    |
| Simon Wright          |
| Mike Salvati          |
| Sasha Goldshtein      |
| Righty Sock           |
| Hannah Steele         |
| Vincent Cadena        |
| Brandt Sponseller     |
| Victor Woeltjen       |
| Krug Stillo           |

---

### 7. Find movies without any rating
**Task:** Write a SQL query to find the movies without any rating. Return movie title.

```sql
SELECT m.mov_title 
FROM movie AS m, movie_rating AS r 
WHERE r.mov_id = m.mov_id 
  AND r.num_o_ratings IS NULL;
```

**Output:**
| mov_title             |
|-----------------------|
| Princess Mononoke     |
| Avatar                |

---

### 8. Find movies with specific IDs
**Task:** Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.

```sql
SELECT mov_title 
FROM movie 
WHERE mov_id IN (905, 907, 917);
```

**Output:**
| mov_title             |
|-----------------------|
|                       |

---

### 9. Find movies containing 'Boogie Nights'
**Task:** Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.

```sql
SELECT mov_id, mov_title, mov_year 
FROM movie 
WHERE mov_title LIKE "%Boogie Nights%" 
ORDER BY mov_year ASC;
```

**Output:**
| mov_id | mov_title      | mov_year |
|--------|----------------|----------|
| 10     | Boogie Nights  | 1997     |

---

### 10. Find actor 'Woody Allen'
**Task:** Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.

```sql
SELECT act_id 
FROM actor 
WHERE act_fname = "Woody" 
  AND act_lname = "Allen";
```

**Output:**
| act_id |
|--------|
| 11     |

---

## Sub-Queries

### 11. Find actors in 'Annie Hall'
**Task:** Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.

```sql
SELECT * 
FROM actor 
WHERE actor.act_id IN (
    SELECT act_id 
    FROM movie_cast c, movie m 
    WHERE c.mov_id = m.mov_id 
      AND m.mov_title = 'Annie Hall'
);
```

**Output:**
| act_id | act_fname | act_lname | act_gender |
|--------|-----------|-----------|------------|
| 11     | Woody     | Allen     | M          |

---

### 12. Find director of 'Eyes Wide Shut'
**Task:** Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.

```sql
SELECT d.dir_fname, d.dir_lname 
FROM director d 
WHERE d.dir_id IN (
    SELECT dir_id 
    FROM movie_direction 
    WHERE mov_id IN (
        SELECT mov_id 
        FROM movie 
        WHERE mov_title = 'Eyes Wide Shut'
    )
);
```

**Output:**
| dir_fname | dir_lname |
|-----------|-----------|
| Stanley   | Kubrick   |

---

### 13. Find movies released outside UK
**Task:** Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.

```sql
SELECT mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country 
FROM movie 
WHERE mov_rel_country != 'UK';
```

**Output:**
| mov_title        | mov_year | mov_time | mov_dt_rel | mov_rel_country |
|------------------|----------|----------|------------|-----------------|
| The Innocents    | 1961     | 100      | 1962-02-19 | SW              |
| Annie Hall       | 1977     | 93       | 1977-04-20 | USA             |
| Seven Samurai    | 1954     | 207      | 1954-04-26 | JP              |

---

### 14. Find movies with unknown reviewer
**Task:** Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

```sql
SELECT m.mov_title,m.mov_year,m.mov_dt_rel,d.dir_fname,d.dir_lname,a.act_fname,a.act_lname FROM 
	movie m LEFT JOIN movie_rating mr ON m.mov_id = mr.mov_id
	JOIN movie_direction md ON m.mov_id = md.mov_id
	JOIN director d ON md.dir_id = d.dir_id
	JOIN movie_cast mc ON m.mov_id = mc.mov_id
	JOIN actor a ON mc.act_id = a.act_id
	WHERE mr.rev_id IS NULL;
```

**Output:**
| mov_title                   | mov_year | mov_dt_rel | dir_fname   | dir_lname | act_fname  | act_lname |
|-----------------------------|----------|------------|-------------|-----------|------------|-----------|
| The Deer Hunter              | 1978     | 1979-03-08 | Michael     | Cimino    | Robert     | De Niro   |
| Amadeus                      | 1984     | 1985-01-07 | Milos       | Forman    | F. Murray  | Abraham  |
| Eyes Wide Shut               | 1999     |            | Stanley     | Kubrick  | Nicole     | Kidman   |
| The Shawshank Redemption     | 1994     | 1995-02-17 | Frank       | Darabont | Tim        | Robbins  |
| Deliverance                  | 1972     | 1982-10-05 | John        | Boorman  | Jon        | Voight   |
| The Prestige                 | 2006     | 2006-11-10 | Christopher | Nolan    | Christian  | Bale     |

---

### 15. Find movies directed by Woody Allen
**Task:** Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.

```sql
SELECT mov_title 
FROM movie 
WHERE mov_id IN (
    SELECT mov_id 
    FROM movie_direction 
    WHERE dir_id IN (
        SELECT dir_id 
        FROM director 
        WHERE dir_fname = 'Woody' 
          AND dir_lname = 'Allen'
    )
);
```

**Output:**
| mov_title   |
|-------------|
| Annie Hall  |
---

### 16. Find years with movies rated 3+ stars
**Task:** Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.

```sql
SELECT DISTINCT mov_year 
FROM movie 
WHERE mov_id IN (
    SELECT mov_id 
    FROM movie_rating 
    WHERE rev_stars >= 3
) 
ORDER BY mov_year;
```

**Output:**
| mov_year |
|----------|
| 1954     |
| 1958     |
| 1961     |
| 1962     |
| 1977     |
| 1982     |
| 1986     |
| 1995     |
| 1997     |
| 1999     |
| 2001     |
| 2004     |
| 2008     |
| 2009     |

---

### 17. Find movies with no ratings
**Task:** Write a SQL query to search for movies that do not have any ratings. Return movie title.

```sql
SELECT mov_title 
FROM movie 
WHERE mov_id NOT IN (
    SELECT mov_id 
    FROM movie_rating
);
```

**Output:**
| mov_title                   |
|-----------------------------|
| The Deer Hunter              |
| Amadeus                      |
| Eyes Wide Shut               |
| The Shawshank Redemption     |
| Deliverance                  |
| The Prestige                 |
| Spirited Away                |
| Back to the Future           |
| Braveheart                   |

---

### 18. Find reviewers who haven't rated certain films
**Task:** Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.

```sql
SELECT rev_name 
FROM movie_reviewer 
WHERE rev_id NOT IN (
    SELECT rev_id 
    FROM movie_rating
);
```

**Output:**
| rev_name            |
|---------------------|
| Alec Shaw           |
| Wesley S. Walker    |

---

### 19. Find reviewed movies with ratings (sorted)
**Task:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.

```sql
SELECT m.mov_title, rev.rev_name, rat.rev_stars 
FROM movie m, movie_reviewer rev, movie_rating rat 
WHERE m.mov_id = rat.mov_id 
  AND rev.rev_id = rat.rev_id 
  AND m.mov_id IN (
      SELECT mov_id 
      FROM movie_rating 
      WHERE rev_id IN (SELECT rev_id FROM movie_reviewer) 
        AND num_o_ratings > 0 
        AND rev_stars > 0
  ) 
ORDER BY rev.rev_name, m.mov_title, rat.rev_stars;
```

**Output:**
| mov_title               | rev_name              | rev_stars |
|-------------------------|-----------------------|-----------|
| Blade Runner            |                       | 8.20      |
| Aliens                  | Brandt Sponseller     | 8.40      |
| Lawrence of Arabia      | Flagrant Baronessa    | 8.30      |
| Donnie Darko            | Hannah Steele         | 8.10      |
| The Innocents           | Jack Malvern          | 7.90      |
| Good Will Hunting       | Josh Cates            | 4.00      |
| Seven Samurai           | Krug Stillo           | 7.70      |
| Annie Hall              | Mike Salvati          | 8.10      |
| Boogie Nights           | Paul Monks            | 3.00      |
| Beyond the Sea          | Richard Adams         | 6.70      |
| Titanic                 | Righty Sock           | 7.70      |
| Vertigo                 | Righty Sock           | 8.40      |
| American Beauty         | Sasha Goldshtein      | 7.00      |
| The Usual Suspects      | Simon Wright          | 8.60      |
| Slumdog Millionaire     | Vincent Cadena        | 8.00      |

---

### 20. Find reviewed movies grouped by reviewer and title
**Task:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer's name, movie title. Return reviewer's name, movie title.

```sql
SELECT m.mov_title, rev.rev_name 
FROM movie m, movie_reviewer rev, movie_rating rat 
WHERE m.mov_id = rat.mov_id 
  AND rev.rev_id = rat.rev_id 
  AND m.mov_id IN (
      SELECT mov_id 
      FROM movie_rating 
      WHERE rev_id IN (SELECT rev_id FROM movie_reviewer) 
        AND num_o_ratings > 0 
        AND rev_stars > 0
  ) 
GROUP BY rev.rev_name, m.mov_title;
```

**Output:**
| mov_title              | rev_name            |
|------------------------|---------------------|
| Vertigo                | Righty Sock         |
| The Innocents          | Jack Malvern        |
| Lawrence of Arabia     | Flagrant Baronessa  |
| Blade Runner           |                     |
| The Usual Suspects     | Simon Wright        |
| Boogie Nights          | Paul Monks          |
| Annie Hall             | Mike Salvati        |
| American Beauty        | Sasha Goldshtein    |
| Titanic                | Righty Sock         |
| Good Will Hunting      | Josh Cates          |
| Donnie Darko           | Hannah Steele       |
| Slumdog Millionaire    | Vincent Cadena      |
| Aliens                 | Brandt Sponseller   |
| Beyond the Sea         | Richard Adams       |
| Seven Samurai          | Krug Stillo         |

---

### 21. Find movies with highest ratings
**Task:** Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.

```sql
SELECT m.mov_title, MAX(r.rev_stars) AS max_stars 
FROM movie m, movie_rating r 
WHERE m.mov_id = r.mov_id 
GROUP BY m.mov_title 
HAVING MAX(r.rev_stars) = (SELECT MAX(rev_stars) FROM movie_rating) 
ORDER BY m.mov_title;
```

**Output:**
| mov_title              | max_stars |
|------------------------|-----------|
| The Usual Suspects     | 8.60      |
---

### 22. Find reviewers who rated 'American Beauty'
**Task:** Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.

```sql
SELECT rev_name 
FROM movie_reviewer 
WHERE rev_id IN (
    SELECT rev_id 
    FROM movie_rating 
    WHERE mov_id IN (
        SELECT mov_id 
        FROM movie 
        WHERE mov_title = 'American Beauty'
    )
);
```

**Output:**
| rev_name           |
|--------------------|
| Sasha Goldshtein   |
---

### 23. Find movies reviewed only by Paul Monks
**Task:** Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.

```sql
SELECT mov_title 
FROM movie 
WHERE mov_id IN (
    SELECT rt.mov_id 
    FROM movie_rating rt, movie_reviewer rv 
    WHERE rt.rev_id = rv.rev_id 
      AND rv.rev_name = 'Paul Monks'
);
```

**Output:**
| mov_title       |
|-----------------|
| Boogie Nights   |

---

### 24. Find movies with lowest ratings
**Task:** Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

```sql
SELECT r.rev_name, m.mov_title, rt.rev_stars 
FROM movie_reviewer r, movie_rating rt, movie m 
WHERE r.rev_id = rt.rev_id 
  AND rt.mov_id = m.mov_id 
  AND m.mov_id IN (
      SELECT mov_id 
      FROM movie_rating 
      WHERE rev_stars IN (SELECT MIN(rev_stars) FROM movie_rating)
  );
```

**Output:**
| rev_name    | mov_title      | rev_stars |
|-------------|----------------|-----------|
| Paul Monks  | Boogie Nights  | 3.00      |
---

### 25. Find movies directed by James Cameron
**Task:** Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.

```sql
SELECT m.mov_title 
FROM movie m, movie_direction md 
WHERE m.mov_id = md.mov_id 
  AND md.dir_id IN (
      SELECT dir_id 
      FROM director 
      WHERE dir_fname = 'James' 
        AND dir_lname = 'Cameron'
  );
```

**Output:**
| mov_title |
|-----------|
| Titanic   |
| Aliens    |
---

### 26. Find movies with actors in multiple films
**Task:** Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

```sql
SELECT m.mov_title 
FROM movie m, movie_cast c 
WHERE m.mov_id = c.mov_id 
  AND c.act_id IN (
      SELECT act_id 
      FROM movie_cast 
      GROUP BY act_id 
      HAVING COUNT(mov_id) > 1
  );
```

**Output:**
| mov_title        |
|------------------|
| American Beauty  |
| Beyond the Sea   |

---

## Joins

### 27. Find reviewers with NULL ratings
**Task:** Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.

```sql
SELECT rv.rev_name 
FROM movie_reviewer rv 
INNER JOIN movie_rating rt ON rv.rev_id = rt.rev_id 
WHERE rt.num_o_ratings IS NULL;
```

**Output:**
| rev_name           |
|--------------------|
|                    |
| Victor Woeltjen    |

---

### 28. Find cast of 'Annie Hall'
**Task:** Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.

```sql
SELECT CONCAT(a.act_fname, ' ', a.act_lname) AS fullname, c.role 
FROM actor a 
JOIN movie_cast c ON a.act_id = c.act_id 
JOIN movie m ON c.mov_id = m.mov_id 
WHERE m.mov_title = 'Annie Hall';
```

**Output:**
| fullname     | role        |
|--------------|-------------|
| Woody Allen  | Alvy Singer |

---

### 29. Find director who acted in 'Eyes Wide Shut'
**Task:** Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.

```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title 
FROM director d 
INNER JOIN actor a ON d.dir_fname = a.act_fname 
  AND d.dir_lname = a.act_lname
INNER JOIN movie_cast c ON a.act_id = c.act_id 
INNER JOIN movie m ON m.mov_id = c.mov_id
WHERE m.mov_title = 'Eyes Wide Shut';
```

**Output:**
| dir_fname | dir_lname | mov_title |
|-----------|-----------|-----------|

---

### 30. Find director of movie with 'Sean Maguire' role
**Task:** Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.

```sql
SELECT d.dir_fname, d.dir_lname, m.mov_title 
FROM movie_cast mc 
JOIN movie_direction md ON mc.mov_id = md.mov_id 
JOIN director d ON d.dir_id = md.dir_id
JOIN movie m ON m.mov_id = mc.mov_id
WHERE mc.role = 'Sean Maguire';
```

**Output:**
| dir_fname | dir_lname | mov_title           |
|-----------|-----------|---------------------|
| Gus       | Van Sant  | Good Will Hunting   |

---

### 31. Find actors not in movies between 1990-2000
**Task:** Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.

```sql
SELECT a.act_fname, a.act_lname, m.mov_title, m.mov_year 
FROM actor a 
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON m.mov_id = mc.mov_id
WHERE m.mov_year NOT BETWEEN 1990 AND 2000;
```

**Output:**
| act_fname | act_lname      | mov_title                | mov_year |
|-----------|----------------|--------------------------|----------|
| James     | Stewart        | Vertigo                  | 1958     |
| Deborah   | Kerr           | The Innocents            | 1961     |
| Peter     | OToole         | Lawrence of Arabia       | 1962     |
| Robert    | De Niro        | The Deer Hunter          | 1978     |
| F. Murray | Abraham        | Amadeus                  | 1984     |
| Harrison  | Ford           | Blade Runner             | 1982     |
| Jack      | Nicholson      | Chinatown                | 1974     |
| Woody     | Allen          | Annie Hall               | 1977     |
| Jon       | Voight         | Deliverance              | 1972     |
| Christian | Bale           | The Prestige             | 2006     |
| Maggie    | Gyllenhaal     | Donnie Darko             | 2001     |
| Dev       | Patel          | Slumdog Millionaire      | 2008     |
| Sigourney | Weaver         | Aliens                   | 1986     |
| Kevin     | Spacey         | Beyond the Sea           | 2004     |

---

### 32. Find directors with variety of genres
**Task:** Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.

```sql
SELECT CONCAT(d.dir_fname, " ", d.dir_lname) AS dir_name, 
       COUNT(mg.gen_id) AS no_of_genres 
FROM director d 
JOIN movie_direction md ON d.dir_id = md.dir_id
JOIN movie_genres mg ON md.mov_id = mg.gen_id
GROUP BY dir_name 
ORDER BY dir_name;
```

**Output:**
| dir_name              | no_of_genres |
|-----------------------|--------------|
| Alfred Hitchcock      | 1            |
| Bryan Singer          | 1            |
| David Lean            | 1            |
| Frank Darabont        | 1            |
| Hayao Miyazaki        | 1            |
| Jack Clayton          | 2            |
| Milos Forman          | 1            |
| Paul Thomas Anderson  | 3            |
| Ridley Scott          | 2            |
| Roman Polanski        | 1            |
| Stanley Kubrick       | 4            |
| Woody Allen           | 1            |

---

### 33. Find movies with year and genres
**Task:** Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.

```sql
SELECT m.mov_title, m.mov_year, g.gen_title 
FROM movie m 
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id;
```

**Output:**
| mov_title                   | mov_year | gen_title |
|-----------------------------|----------|-----------|
| Aliens                      | 1986     | Action    |
| Lawrence of Arabia           | 1962     | Adventure |
| Deliverance                 | 1972     | Adventure |
| Princess Mononoke           | 1997     | Animation |
| Annie Hall                  | 1977     | Comedy    |
| The Usual Suspects          | 1995     | Crime     |
| The Shawshank Redemption    | 1994     | Crime     |
| Trainspotting               | 1996     | Drama     |
| Slumdog Millionaire         | 2008     | Drama     |
| Spirited Away               | 2001     | Drama     |
| Braveheart                  | 1995     | Drama     |
| The Innocents               | 1961     | Horror    |
| Beyond the Sea              | 2004     | Music     |
| Vertigo                     | 1958     | Mystery   |
| Eyes Wide Shut              | 1999     | Mystery   |
| Back to the Future          | 1985     | Mystery   |
| American Beauty             | 1999     | Romance   |
| Blade Runner                | 1982     | Thriller  |
| The Deer Hunter             | 1978     | War       |

---

### 34. Find movies with year, genres, and director
**Task:** Write a SQL query to find all the movies with year, genres, and name of the director.

```sql
SELECT m.mov_title, m.mov_year, g.gen_title, 
       CONCAT(d.dir_fname, " ", d.dir_lname) AS dir_name 
FROM movie m 
JOIN movie_genres mg ON m.mov_id = mg.mov_id
JOIN genres g ON mg.gen_id = g.gen_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON d.dir_id = md.dir_id;
```

**Output:**
| mov_title                   | mov_year | gen_title | dir_name           |
|-----------------------------|----------|-----------|--------------------|
| Aliens                      | 1986     | Action    | James Cameron      |
| Lawrence of Arabia           | 1962     | Adventure | David Lean         |
| Deliverance                 | 1972     | Adventure | John Boorman       |
| Princess Mononoke           | 1997     | Animation | Hayao Miyazaki     |
| Annie Hall                  | 1977     | Comedy    | Woody Allen        |
| The Usual Suspects          | 1995     | Crime     | Bryan Singer       |
| The Shawshank Redemption    | 1994     | Crime     | Frank Darabont     |
| Trainspotting               | 1996     | Drama     | Danny Boyle        |
| Slumdog Millionaire         | 2008     | Drama     | Danny Boyle        |
| The Innocents               | 1961     | Horror    | Jack Clayton       |
| Beyond the Sea              | 2004     | Music     | Kevin Spacey       |
| Vertigo                     | 1958     | Mystery   | Alfred Hitchcock   |
| Eyes Wide Shut               | 1999     | Mystery   | Stanley Kubrick    |
| American Beauty             | 1999     | Romance   | Sam Mendes         |
| Blade Runner                | 1982     | Thriller | Ridley Scott       |
| The Deer Hunter             | 1978     | War       | Michael Cimino     |

---

### 35. Find movies released before January 1, 1989
**Task:** Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.

```sql
SELECT m.mov_title, YEAR(m.mov_dt_rel) AS release_year, 
       m.mov_dt_rel, m.mov_time, 
       CONCAT(d.dir_fname, " ", d.dir_lname) AS dir_name 
FROM movie m
JOIN movie_direction md ON md.mov_id = m.mov_id
JOIN director d ON d.dir_id = md.dir_id
WHERE m.mov_dt_rel < '1989-01-01'
ORDER BY m.mov_dt_rel DESC;
```

**Output:**
| mov_title              | release_year | mov_dt_rel | mov_time | dir_name            |
|------------------------|--------------|------------|----------|---------------------|
| Aliens                 | 1986         | 1986-08-29 | 137      | James Cameron       |
| Amadeus                | 1985         | 1985-01-07 | 160      | Milos Forman        |
| Deliverance            | 1982         | 1982-10-05 | 109      | John Boorman        |
| Blade Runner           | 1982         | 1982-09-09 | 117      | Ridley Scott        |
| The Deer Hunter        | 1979         | 1979-03-08 | 183      | Michael Cimino      |
| Annie Hall             | 1977         | 1977-04-20 | 93       | Woody Allen         |
| Chinatown              | 1974         | 1974-08-09 | 130      | Roman Polanski      |
| Lawrence of Arabia     | 1962         | 1962-12-11 | 216      | David Lean          |
| The Innocents          | 1962         | 1962-02-19 | 100      | Jack Clayton        |
| Vertigo                | 1958         | 1958-08-24 | 128      | Alfred Hitchcock    |

---

### 36. Calculate average movie length by genre
**Task:** Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

```sql
SELECT g.gen_title, AVG(m.mov_time) AS average_length, 
       COUNT(m.mov_time) AS count 
FROM movie m
JOIN movie_genres mg ON mg.mov_id = m.mov_id
JOIN genres g ON g.gen_id = mg.gen_id
GROUP BY g.gen_title;
```

**Output:**
| gen_title | average_length | count |
|-----------|----------------|-------|
| Action    | 137.0000       | 1     |
| Adventure | 162.5000       | 2     |
| Animation | 134.0000       | 1     |
| Comedy    | 93.0000        | 1     |
| Crime     | 124.0000       | 2     |
| Drama     | 129.2500       | 4     |
| Horror    | 100.0000       | 1     |
| Music     | 118.0000       | 1     |
| Mystery   | 134.3333       | 3     |
| Romance   | 122.0000       | 1     |
| Thriller  | 117.0000       | 1     |
| War       | 183.0000       | 1     |
---

### 37. Find movies with shortest duration
**Task:** Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.

```sql
SELECT m.mov_title, m.mov_year, 
       CONCAT(d.dir_fname, " ", d.dir_lname) AS dir_name, 
       CONCAT(a.act_fname, " ", a.act_lname) AS act_name, 
       mc.role 
FROM movie m
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON d.dir_id = md.dir_id
JOIN movie_cast mc ON mc.mov_id = m.mov_id
JOIN actor a ON a.act_id = mc.act_id
WHERE m.mov_time = (SELECT MIN(mov_time) FROM movie);
```

**Output:**
| mov_title   | mov_year | dir_name    | act_name    | role        |
|-------------|----------|-------------|-------------|-------------|
| Annie Hall  | 1977     | Woody Allen | Woody Allen | Alvy Singer |

---

### 38. Find years with 3 or 4 star ratings
**Task:** Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.

```sql
SELECT m.mov_year 
FROM movie m 
JOIN movie_rating mr ON mr.mov_id = m.mov_id
WHERE mr.rev_stars = 3 OR mr.rev_stars = 4
ORDER BY m.mov_year;
```

**Output:**
| mov_year |
|----------|
| 1997     |
| 1997     |

---

### 39. Get reviewer name, movie title, and stars
**Task:** Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.

```sql
SELECT mrv.rev_name, m.mov_title, mrt.rev_stars 
FROM movie m
JOIN movie_rating mrt ON m.mov_id = mrt.mov_id
JOIN movie_reviewer mrv ON mrv.rev_id = mrt.rev_id;
```

**Output:**
| rev_name             | mov_title              | rev_stars |
|----------------------|------------------------|-----------|
| Righty Sock          | Vertigo                | 8.40      |
| Jack Malvern         | The Innocents          | 7.90      |
| Flagrant Baronessa   | Lawrence of Arabia     | 8.30      |
|                      | Blade Runner           | 8.20      |
| Simon Wright         | The Usual Suspects     | 8.60      |
| Neal Wruck           | Chinatown              |           |
| Paul Monks           | Boogie Nights          | 3.00      |
| Mike Salvati         | Annie Hall             | 8.10      |
|                      | Princess Mononoke      | 8.40      |
| Sasha Goldshtein     | American Beauty        | 7.00      |
| Righty Sock          | Titanic                | 7.70      |
| Josh Cates           | Good Will Hunting      | 4.00      |
| Scott LeBrun         | Trainspotting          |           |
| Hannah Steele        | Donnie Darko           | 8.10      |
| Vincent Cadena       | Slumdog Millionaire    | 8.00      |
| Brandt Sponseller    | Aliens                 | 8.40      |
| Richard Adams        | Beyond the Sea         | 6.70      |
| Victor Woeltjen      | Avatar                 | 7.30      |
| Krug Stillo          | Seven Samurai          | 7.70      |

---

### 40. Find movies with highest ratings
**Task:** Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.

```sql
SELECT m.mov_title, mr.rev_stars 
FROM movie m
JOIN movie_rating mr ON mr.mov_id = m.mov_id
WHERE mr.rev_stars = (SELECT MAX(rev_stars) FROM movie_rating) 
  AND mr.num_o_ratings > 0
ORDER BY m.mov_title;
```

**Output:**
| mov_title              | rev_stars |
|------------------------|-----------|
| The Usual Suspects     | 8.60      |


---

### 41. Find movies that received ratings
**Task:** Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.

```sql
SELECT m.mov_title, CONCAT(d.dir_fname, " ", d.dir_lname) AS dir_name, 
       mr.rev_stars 
FROM movie m
JOIN movie_rating mr ON mr.mov_id = m.mov_id
JOIN movie_direction md ON md.mov_id = m.mov_id
JOIN director d ON d.dir_id = md.dir_id
WHERE mr.num_o_ratings > 0;
```

**Output:**
| mov_title              | dir_name              | rev_stars |
|------------------------|-----------------------|-----------|
| Vertigo                | Alfred Hitchcock      | 8.40      |
| The Innocents          | Jack Clayton          | 7.90      |
| Lawrence of Arabia     | David Lean            | 8.30      |
| Blade Runner           | Ridley Scott          | 8.20      |
| The Usual Suspects     | Bryan Singer          | 8.60      |
| Chinatown              | Roman Polanski        |           |
| Boogie Nights          | Paul Thomas Anderson  | 3.00      |
| Annie Hall             | Woody Allen           | 8.10      |
| American Beauty        | Sam Mendes            | 7.00      |
| Titanic                | James Cameron         | 7.70      |
| Good Will Hunting      | Gus Van Sant          | 4.00      |
| Trainspotting          | Danny Boyle           |           |
| Donnie Darko           | Richard Kelly         | 8.10      |
| Slumdog Millionaire    | Danny Boyle           | 8.00      |
| Aliens                 | James Cameron         | 8.40      |
| Beyond the Sea         | Kevin Spacey          | 6.70      |


---

### 42. Find movies with actors in multiple films
**Task:** Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.

```sql
SELECT m.mov_title, CONCAT(a.act_fname, " ", a.act_lname) AS act_name, 
       mc.role 
FROM movie m
JOIN movie_cast mc ON mc.mov_id = m.mov_id
JOIN actor a ON a.act_id = mc.act_id
WHERE mc.act_id IN (
    SELECT act_id 
    FROM movie_cast 
    GROUP BY act_id 
    HAVING COUNT(mov_id) > 1
);
```

**Output:**
| mov_title        | act_name      | role            |
|------------------|---------------|-----------------|
| American Beauty  | Kevin Spacey  | Lester Burnham  |
| Beyond the Sea   | Kevin Spacey  | Bobby Darin     |

---

### 43. Find movies with Claire Danes
**Task:** Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.

```sql
SELECT CONCAT(d.dir_fname, " ", d.dir_lname) AS dir_name, 
       m.mov_title, 
       CONCAT(a.act_fname, " ", a.act_lname) AS act_name, 
       mc.role 
FROM actor a 
JOIN movie_cast mc ON mc.act_id = a.act_id
JOIN movie m ON m.mov_id = mc.mov_id
JOIN movie_direction md ON md.mov_id = mc.mov_id
JOIN director d ON d.dir_id = md.dir_id
WHERE a.act_fname = 'Claire' AND a.act_lname = 'Danes';
```

**Output:**
| dir_name        | mov_title            | act_name      | role |
|-----------------|----------------------|---------------|------|
| Hayao Miyazaki  | Princess Mononoke    | Claire Danes  | San  |

---

### 44. Find actors who directed their own films
**Task:** Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.

```sql
SELECT CONCAT(a.act_fname, " ", a.act_lname) AS act_name, 
       m.mov_title, mc.role 
FROM actor a
JOIN movie_cast mc ON a.act_id = mc.act_id
JOIN movie m ON mc.mov_id = m.mov_id
JOIN movie_direction md ON m.mov_id = md.mov_id
JOIN director d ON md.dir_id = d.dir_id
WHERE a.act_fname = d.dir_fname AND a.act_lname = d.dir_lname;
```

**Output:**
| act_name      | mov_title        | role        |
|---------------|------------------|-------------|
| Woody Allen   | Annie Hall       | Alvy Singer |
| Kevin Spacey  | Beyond the Sea   | Bobby Darin |

---

### 45. Find cast list of 'Chinatown'
**Task:** Write a SQL query to find the cast list of the movie 'Chinatown'. Return first name, last name.

```sql
SELECT CONCAT(a.act_fname, " ", a.act_lname) AS act_name 
FROM actor a
JOIN movie_cast mc ON mc.act_id = a.act_id
JOIN movie m ON m.mov_id = mc.mov_id
WHERE m.mov_title = 'Chinatown';
```

**Output:**
| act_name        |
|-----------------|
| Jack Nicholson  |


---

### 46. Find movies with Harrison Ford
**Task:** Write a SQL query to find those movies where actor's first name is 'Harrison' and last name is 'Ford'. Return movie title.

```sql
SELECT m.mov_title 
FROM movie m 
JOIN movie_cast mc ON m.mov_id = mc.mov_id
JOIN actor a ON a.act_id = mc.act_id
WHERE a.act_fname = 'Harrison' AND a.act_lname = 'Ford';
```

**Output:**
| mov_title     |
|---------------|
| Blade Runner  |


---

### 47. Find highest-rated movies
**Task:** Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.

```sql
SELECT m.mov_title, m.mov_year, mr.rev_stars, m.mov_rel_country 
FROM movie m
JOIN movie_rating mr ON m.mov_id = mr.mov_id
WHERE mr.rev_stars in (SELECT MAX(rev_stars) FROM movie_rating);
```

**Output:**
| mov_title              | mov_year | rev_stars | mov_rel_country |
|------------------------|----------|-----------|-----------------|
| The Usual Suspects     | 1995     | 8.60      | UK              |


---

### 48. Find highest-rated Mystery Movies
**Task:** Write a SQL query to find the highest-rated 'Mystery Movies'. Return the title, year, and rating.

```sql
SELECT m.mov_title, m.mov_year, mr.rev_stars 
FROM movie m 
JOIN movie_rating mr ON m.mov_id = mr.mov_id
JOIN movie_genres mg ON mg.mov_id = m.mov_id
JOIN genres gn ON mg.gen_id = gn.gen_id
WHERE gn.gen_title = 'Mystery'
GROUP BY m.mov_title, m.mov_year, mr.rev_stars 
HAVING mr.rev_stars = MAX(mr.rev_stars);
```

**Output:**
| mov_title | mov_year | rev_stars |
|-----------|----------|-----------|
| Vertigo   | 1958     | 8.40      |


---

### 49. Analyze Mystery Movies by year
**Task:** Write a SQL query to find the years when most of the 'Mystery Movies' produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.

```sql
SELECT m.mov_year, g.gen_title, count(m.mov_id), avg(mrt.rev_stars) 
	FROM movie m
	JOIN movie_genres mg ON m.mov_id = mg.mov_id
	JOIN genres g ON mg.gen_id = g.gen_id
	LEFT JOIN movie_rating mrt ON m.mov_id = mrt.mov_id
	WHERE g.gen_title = 'mystery'
    GROUP BY m.mov_year, g.gen_title;
```

**Output:**
| mov_year | gen_title | count(mv.mov_id) | avg(mrt.rev_stars) |
|----------|-----------|------------------|--------------------|
| 1958     | Mystery   | 1                | 8.400000           |
| 1999     | Mystery   | 1                |                    |
| 1985     | Mystery   | 1                |                    |

---

### 50. Generate comprehensive movie report for female actors
**Task:** Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

```sql
SELECT m.mov_title, concat(a.act_fname, " ", a.act_lname) act_name, m.mov_year, mc.role, g.gen_title, concat(d.dir_fname, " ", d.dir_lname) dir_name, m.mov_dt_rel, mrt.rev_stars 
    from movie m
	JOIN movie_cast mc ON m.mov_id = mc.mov_id
	JOIN actor a ON a.act_id = mc.act_id
	JOIN movie_genres mg ON mg.mov_id = m.mov_id
	JOIN genres g ON g.gen_id = mg.gen_id
	JOIN movie_direction md ON md.mov_id = m.mov_id
	JOIN director d ON d.dir_id = md.dir_id
	LEFT JOIN movie_rating mrt ON mrt.mov_id = m.mov_id
	WHERE a.act_gender = 'f';

```

**Output:**
| mov_title           | act_name          | mov_year | role           | gen_title  | dir_name          | mov_dt_rel  | rev_stars |
|---------------------|-------------------|----------|----------------|------------|-------------------|-------------|-----------|
| The Innocents       | Deborah Kerr      | 1961     | Miss Giddens   | Horror     | Jack Clayton      | 1962-02-19  | 7.90      |
| Eyes Wide Shut      | Nicole Kidman     | 1999     | Alice Harford  | Mystery    | Stanley Kubrick   |             |           |
| Princess Mononoke   | Claire Danes      | 1997     | San            | Animation  | Hayao Miyazaki    | 2001-10-19  | 8.40      |
| Aliens              | Sigourney Weaver  | 1986     | Ripley         | Action     | James Cameron     | 1986-08-29  | 8.40      |


---