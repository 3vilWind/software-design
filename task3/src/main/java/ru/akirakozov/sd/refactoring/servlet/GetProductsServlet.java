package ru.akirakozov.sd.refactoring.servlet;

import ru.akirakozov.sd.refactoring.model.Product;
import ru.akirakozov.sd.refactoring.service.ProductService;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;

/**
 * @author akirakozov
 */
public class GetProductsServlet extends HttpServlet {
    protected ProductService productService;

    public GetProductsServlet(ProductService productService) {
        this.productService = productService;
    }

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        response.getWriter().println("<html><body>");

        for(Product product: productService.getAllProducts()) {
            response.getWriter().println(product.name + "\t" + product.price + "</br>");
        }

        response.getWriter().println("</body></html>");

        response.setContentType("text/html");
        response.setStatus(HttpServletResponse.SC_OK);
    }
}
