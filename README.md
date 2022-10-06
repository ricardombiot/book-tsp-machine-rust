# TSP Machine in Rust

En este repositorio podemos encontrarnos la migración a Rust del algoritmo que presentamos en nuestro libro [“P vs. NP - Abstracciones Exponenciales: Aplicación de las abstracciones exponenciales a los problemas HC & TSP”](https://ricardombiot.com/libros/abstracciones_exponenciales/)([Amazon](https://amzn.to/3CvudQZ), [Google](https://play.google.com/store/books/details?id=GEmJEAAAQBAJ)). 


![P vs. NP - Abstracciones Exponenciales: Aplicación de las abstracciones exponenciales a los problemas HC & TSP](https://ricardombiot.com/static/abstracciones_exponenciales_frontcover_forweb.png)

Cabe mencionar que posteriormente creamos un [fork]( https://github.com/ricardombiot/book-tsp-machine-binset-rust) de esta versión usando “Binary Sets” (conjuntos binarios que implementados a bajo nivel que hacen uso de operaciones bitwise) con el fin de probar si esa implementación de los conjuntos podría ayudarnos a reducir los elevados costes espaciales que presenta por default nuestro algortimo.



Es importante destacar como, a pesar de realizar la migración a Rust, hemos intentado conservar en la medida de lo posible, por fines didácticos, el mismo estilo arquitectónico del [código en Julia](https://github.com/ricardombiot/book-tsp-machine-julia).

<sub>Advertencia: Esta versión puede incluir muchos spaiks y código muerto. </sub>
