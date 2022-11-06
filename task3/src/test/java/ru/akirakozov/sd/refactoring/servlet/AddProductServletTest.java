package ru.akirakozov.sd.refactoring.servlet;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mock;
import org.mockito.MockitoAnnotations;
import ru.akirakozov.sd.refactoring.Utils;
import ru.akirakozov.sd.refactoring.gateways.ProductRepository;
import ru.akirakozov.sd.refactoring.service.ProductService;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.io.PrintWriter;
import java.io.StringWriter;
import java.sql.Connection;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;

import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

public class AddProductServletTest {
    @Mock
    private HttpServletRequest request;
    @Mock
    private HttpServletResponse response;

    private AddProductServlet servlet;

    @BeforeEach
    void setUp() throws SQLException {
        MockitoAnnotations.openMocks(this);
        servlet = new AddProductServlet(Utils.getProductServiceForTestDatabase());
        Utils.cleanTestDatabase();
    }

    @Test
    void testAddOneProduct() throws IOException, SQLException {
        when(request.getParameter("name")).thenReturn("cookies");
        when(request.getParameter("price")).thenReturn("100");
        StringWriter stringWriter = new StringWriter();
        when(response.getWriter()).thenReturn(new PrintWriter(stringWriter));

        servlet.doGet(request, response);

        Assertions.assertEquals(stringWriter.toString().trim(), "OK");
        verify(response).setStatus(HttpServletResponse.SC_OK);

        try (Connection c = Utils.getTestDatabase()) {
            Statement stmt = c.createStatement();
            ResultSet rs = stmt.executeQuery("SELECT * FROM PRODUCT");

            Assertions.assertTrue(rs.next());
            Assertions.assertEquals("cookies", rs.getString("name"));
            Assertions.assertEquals(100, rs.getInt("price"));
            Assertions.assertFalse(rs.next());

            rs.close();
            stmt.close();
        }
    }
}
